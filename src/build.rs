use genanki_rs::{Deck, Field, Model, Note, Package, Template};
use std::fs;
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

fn make_package(input: &Path) -> Result<Package, genanki_rs::Error> {
    let images: Vec<PathBuf> = get_images(input)?;

    let mandocard_model = Model::new(
        1607392319, // figure out the correct value here
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
        // get just the names here...so gross
        let front = pair.0.split("/").last().unwrap();
        let back = pair.1.split("/").last().unwrap();
        // print so i can try to troubleshoot bugs with pairing
        println!("{front} -> {back}");
        deck.add_note(Note::new(
            mandocard_model.clone(),
            vec![
                &format!("<img src={front:?}/>"),
                &format!("<img src={back:?}/>"),
            ],
        )?);

        // go ahead and save the image paths while we're iterating because we need to add them to the package
        image_paths.push(pair.0);
        image_paths.push(pair.1);
    }

    Ok(Package::new(
        vec![deck],
        image_paths//images.iter().map(|i| i.to_str().unwrap()).collect(),
    )?)
}

// this function is so janky...
fn create_pairs(images: &Vec<PathBuf>) -> Result<Vec<(&str, &str)>, Error> {
    // partition by "front" (...and assume "back")...sigh
    let (mut fronts, mut backs): (_, Vec<_>) = 
        images.into_iter().filter_map(|i| i.to_str()).partition(|i| i.contains("front"));

    // sort the front and back so they're in the same order. blah.
    fronts.sort();
    backs.sort();

    // use the index of front to match the back. yuck...
    let mut pairs: Vec<(&str, &str)> = vec![];
    for (i, f) in fronts.iter().enumerate() {
        pairs.push((*f, backs[i]))
    }

    Ok(pairs)
}
