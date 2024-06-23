use genanki_rs::{Deck, Error as AnkiError, Field, Model, Note, Package, Template};
use std::fs;
use std::ffi::OsStr;
use std::io::Error;
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

fn paths_to_filename<'front, 'back>(pair: (&'front PathBuf, &'back PathBuf)) -> Result<(&'front OsStr, &'back OsStr), AnkiError> {
    let front = match pair.0.file_name() {
        Some(f) => f,
        None => {
            return Err(AnkiError::Io(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid value for 'front': {f:?}",
            )))
        }
    };

    let back = match pair.1.file_name() {
        Some(b) => b,
        None => {
            return Err(AnkiError::Io(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid value for 'back': {b:?}",
            )))
        }
    };

    Ok((front, back))
}

fn paths_to_str<'a>(pair: (&'a PathBuf, &'a PathBuf)) -> Result<Vec<&str>, AnkiError> {
    let front = match pair.0.to_str() {
        Some(f) => f,
        None => {
            return Err(AnkiError::Io(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid value for 'front': {f:?}",
            )))
        }
    };

    let back = match pair.1.to_str() {
        Some(b) => b,
        None => {
            return Err(AnkiError::Io(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid value for 'back': {b:?}",
            )))
        }
    };

    Ok(vec!(front, back))
}

fn make_package(input: &Path) -> Result<Package, AnkiError> {
    let images: Vec<PathBuf> = get_images(input)?;

    let mandocard_model = Model::new(
        1607392319, // TODO: figure out the correct value here
        "Mandocard",
        vec![Field::new("Front"), Field::new("Back")],
        vec![Template::new("Mandolin Note")
            .qfmt("{{Front}}")
            .afmt("{{Back}}")],
    );

    let mut deck = Deck::new(1234, "Mandolin", "Mandolin Notes & Frets");

    let pairs = create_pairs(&images)?;
    let mut image_paths: Vec<&str> = vec![];

    for pair in pairs {
        let (front, back) = match paths_to_filename(pair) {
            Ok((f,b)) => (f,b),
            Err(e) => return Err(e)
        };
     
        // print so i can try to troubleshoot bugs with pairing
        println!("{front:?} -> {back:?}");
        deck.add_note(Note::new(
            mandocard_model.clone(),
            vec![
                &format!("<img src={front:?}/>"),
                &format!("<img src={back:?}/>"),
            ],
        )?);

        match paths_to_str(pair) {
            // there must be a better way to do this...
            Ok(p) => for i in p { image_paths.push(i); println!("path: {}", i) },
            Err(e) => return Err(e)
        };
    }

    Ok(Package::new(
        vec![deck],
        image_paths, //images.iter().map(|i| i.to_str().unwrap()).collect(),
    )?)
}

// this function is so janky...
fn create_pairs(images: &Vec<PathBuf>) -> Result<Vec<(&PathBuf, &PathBuf)>, Error> {
    // partition by "front" (...and assume "back")...sigh
    // let (mut fronts, mut backs): (_, Vec<_>) =
    //     images.into_iter().filter_map(|i| i.to_str()).partition(|i| i.contains("front"));

    let (mut fronts, mut backs): (_, Vec<_>) = images
        .into_iter()
        .partition(|f| f.to_str().unwrap().contains("front"));

    // sort the front and back so they're in the same order. blah.
    fronts.sort();
    backs.sort();

    // use the index of front to match the back. yuck...
    let mut pairs: Vec<(&PathBuf, &PathBuf)> = vec![];
    for (i, f) in fronts.iter().enumerate() {
        pairs.push((*f, backs[i]))
    }

    Ok(pairs)
}
