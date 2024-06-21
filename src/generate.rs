use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader, Error, Write};
use std::path::PathBuf;

#[derive(Deserialize)]
struct MandolinNote {
    name: String,
    value: String,
    frets: Vec<String>,
    clef: Option<String>,
}

pub fn score(input_file_path: &PathBuf, output_file_path: &PathBuf) -> Result<(), Error> {
    let input = File::open(input_file_path)?;
    let reader = BufReader::new(input);

    // Deserialize the mandolin definition into Vector of MandolinNote
    let mandolin: Vec<MandolinNote> = serde_json::from_reader(reader)?;

    let mut output = match try_get_output_writer(output_file_path) {
        Ok(writer) => writer,
        Err(e) => panic!("Could not create output writer: {e:?}"),
    };

    write!(
        output,
        "{}",
        create_score("\\version \"2.24.1\"", &mandolin)
    )
}

fn create_score(header: &str, mandolin: &Vec<MandolinNote>) -> String {
    mandolin.iter().fold(String::from(header), |notes, note| {
        notes + &create_note_entry(note)
    })
}

fn try_get_output_writer(output_file_path: &PathBuf) -> Result<Box<dyn Write>, Error> {
    if *output_file_path == PathBuf::from("-") {
        Ok(Box::new(io::stdout().lock()) as Box<dyn Write>)
    } else {
        File::create(output_file_path).map(|f| Box::new(f) as Box<dyn Write>)
    }
}

fn create_note_entry(note: &MandolinNote) -> String {
    let note_name: &str = &note.name;
    let clef: &str = match &note.clef {
        Some(c) => c,
        _ => "treble",
    };
    let note_value: &str = &note.value;
    let note_name_as_accidental: &str = &note_name
        .replace("b", "\\super \\flat")
        .replace("#", "\\super \\sharp");
    let fret_diagram: &str = &note
        .frets
        .iter()
        .fold(String::new(), |frets: String, f: &String| frets + f + ";");
    let frets_to_string_names: &Vec<String> = &note
        .frets
        .iter()
        .map(|f| {
            f.replace("4-", "G-")
                .replace("3-", "D-")
                .replace("2-", "A-")
                .replace("1-", "E-")
        })
        .collect();
    let fret_names: &str = &frets_to_string_names.join(", ");

    format!(
        r#"
          \book {{
            \bookOutputName "{note_name}"
            \bookOutputSuffix "front"
            \score {{
              \new Staff {{
                \omit Staff.BarLine
                \omit Staff.TimeSignature
                \clef "{clef}"
                \new Voice {{
                  {{ {note_value} }}
                }}
              }}
            }}
          }}
      
          \book {{
            \bookOutputName "{note_name}"
            \bookOutputSuffix "back"
            \markup {{
              \center-column {{
                % Note Name
                \concat {{ {note_name_as_accidental} }}
                \override #'(fret-diagram-details . (
                            (string-count . 4)
                            (number-type . arabic)
                            (fret-count . 20)
                            (open-string . '0')
                            (dot-color . red)
                ))
                % Fret Diagram
                \fret-diagram "{fret_diagram}"
                % Fret Names
                \fontsize #-5 "{fret_names}"
              }}
            }}
          }}
        "#
    )
}
