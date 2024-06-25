use genanki_rs::{Deck, Error as AnkiError, Field, Model, Note, Package, Template};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

// THIS IS A MESS!
// Hacked this together just to make the deck build work but I have a
// lot of Rust to learn to make this suck a lot less...
pub fn deck(input: &Path, output: &Path) -> Result<(), Error> {
    let deck = output.to_str().expect("Failed to construct Anki deck file");
    make_package(input)
        .expect("Creating the package failed")
        .write_to_file(deck)
        .expect("Building the package failed");

    Ok(())
}

// this is pretty weak but seems to be doing the job for now
fn get_images(input: &Path) -> Result<Vec<PathBuf>, Error> {
    Ok(fs::read_dir(input)?
        .filter_map(|d| d.ok())
        .map(|d| d.path())
        .filter_map(|p| {
            if p.extension().map_or(false, |ext| ext == "png") {
                Some(p)
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}

fn error<T>(_m: &str, _e: T) -> AnkiError {
    AnkiError::Io(Error::new(
        std::io::ErrorKind::InvalidInput,
        "Invalid value for '{_v}': {_e:?}",))
}

fn paths_to_filename<'front, 'back>(
    pair: (&'front PathBuf, &'back PathBuf),
) -> Result<(&'front str, &'back str), AnkiError> {

    let front = match pair.0.file_name() {
        Some(f) => 
            match f.to_str() {
                Some(s) => s,
                None => return Err(error("front", f))
            }
        None => { return Err(error("front", pair.0)); }
    };

    let back = match pair.1.file_name() {
        Some(b) => 
            match b.to_str() {
                Some(s) => s,
                None => { return Err(error("back", b)); }
            }
        None => { return Err(error("back", pair.1)); }
    };


    Ok((front, back))
}

fn paths_to_str<'a>(pair: (&'a PathBuf, &'a PathBuf)) -> Result<(&str, &str), AnkiError> {
    let front = match pair.0.to_str() {
        Some(f) => f, 
        None => { return Err(error("front", pair.0)); }
    };

    let back = match pair.1.to_str() {
        Some(b) => b,
        None => { return Err(error("back", pair.1)); }
    };

    Ok((front, back))
}

fn make_package(input: &Path) -> Result<Package, AnkiError> {
    let images: Vec<PathBuf> = get_images(input)?;
    let pairs = match create_pairs(&images) {
        Some(p) => p,
        None => {
            return Err(AnkiError::Io(
                Error::new(ErrorKind::InvalidInput, "No image files for deck"),
            ))
        }
    };

    let mut image_paths: Vec<&str> = Vec::new();
    let mut deck = Deck::new(1234, "Mandolin", "Mandolin Notes & Frets");
    let mandocard_model = Model::new(
        1607392319, // TODO: figure out the correct value here
        "Mandocard",
        vec![Field::new("Front"), Field::new("Back")],
        vec![Template::new("Mandolin Note")
            .qfmt("{{Front}}")
            .afmt("{{Back}}")],
    );

    for pair in pairs {
        let (front, back) = paths_to_filename(pair)?;

        deck.add_note(Note::new(
            mandocard_model.clone(),
            vec![
                &format!("<img src={front:?}/>"),
                &format!("<img src={back:?}/>"),
            ],
        )?);

        let (f_path, b_path) = paths_to_str(pair)?;

        image_paths.push(f_path);
        image_paths.push(b_path);
    }

    Ok(Package::new(vec![deck], image_paths)?)
}

// this function is so janky...
fn create_pairs(images: &Vec<PathBuf>) -> Option<Vec<(&PathBuf, &PathBuf)>> {
    // split the vector of paths into "fronts" and "backs" - assumes that all non-fronts are backs.
    let (mut fronts, mut backs): (_, Vec<_>) = images
        .into_iter()
        .partition(|f| { 
            match f.to_str() {
                Some(p) => p.contains("front"),
                _ => false
            }
        });

    // sort the front and back so they're in the same order. blah.
    fronts.sort();
    backs.sort();

    // use the index of front to match the back. yuck...
    let mut pairs: Vec<(&PathBuf, &PathBuf)> = Vec::new();
    for (i, f) in fronts.iter().enumerate() {
        pairs.push((*f, backs[i]))
    }

    if pairs.is_empty() { return None; }
    
    Some(pairs)
}
