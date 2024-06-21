//use genanki_rs::{Note, Error};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write, BufReader, Error };
use clap::Parser;

#[derive(Deserialize, Serialize)]
struct MandolinNote {
    name: String,
    value: String,
    frets: Vec<String>,
    clef: Option<String>
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value="mandolin.json")]
    input: String,
    
    #[arg(short, long, default_value="-")]
    output: String,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let input = File::open(args.input)?;
    let reader = BufReader::new(input);
    let output_file_path = &args.output;

    // Deserialize the mandolin definition into Vector of MandolinNote
    let mandolin: Vec<MandolinNote>= serde_json::from_reader(reader)?;

    let mut output = match try_get_output_writer(output_file_path) {
        Ok(writer) => writer,
        Err(e) => panic!("Error: {e:?}"),
    };
    
    write!(output, "{}", create_score("\\version \"2.24.1\"", &mandolin))
      .expect("Could not save score");

    Ok(())
}

fn create_score(header: &str, mandolin: &Vec<MandolinNote>) -> String {
  return mandolin.iter().fold(String::from(header), |notes, note| notes + &create_note_entry(note));
}

fn try_get_output_writer(output_file_name: &str) -> Result<Box<dyn Write>, Error> {
    let writer = match output_file_name {
        "-" => Ok(Box::new(io::stdout().lock()) as Box<dyn Write>),
        _ => File::create(output_file_name).map(|f| Box::new(f) as Box<dyn Write>),
    };

    return writer;
}

fn create_note_entry(note: &MandolinNote) -> String {
    let note_name: &String = &note.name;
    let clef: &String = match &note.clef {
      Some(c) => c,
      _ => &String::from("treble")
    };
    let note_value: &String = &note.value;
    let note_name_as_accidental: &String = &note_name.replace("b", "\\super \\flat").replace("#", "\\super \\sharp");
    let fret_diagram: &String = &note.frets.iter().fold(String::new(), |frets: String, f: &String| frets + f + ";");
    let frets_to_string_names: &Vec<String> = &note.frets.iter().map(|f| { f.replace("4-", "G-").replace("3-", "D-").replace("2-", "A-").replace("1-", "E-") }).collect();
    let fret_names: &String = &frets_to_string_names.join(", ");

    return format!(r#"
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
    "#);
}