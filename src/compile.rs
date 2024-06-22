use std::io::Error;
use std::path::Path;
use std::process::Command;

pub fn images(input: &Path, output: &Path) -> Result<(), Error> {
    const LILYPOND_COMMAND: &str = "lilypond";

    // This is note successfully creating any of the image files. The command seems to run successfully, but it
    // fails to produce the PNG files. I'm a bit stumped and since this is something I can easily run myself,
    // I'm leaving this as is for now to work on the Deck building code.
    // lilypond --png -d crop="#t" -d resolution="400" -dno-print-pages --output <path>/ mandocard.ly
    match Command::new(LILYPOND_COMMAND)
        .current_dir(output.canonicalize()?)
        .args([
            // "--verbose",
            "--output",
            output
                .canonicalize()?
                .to_str()
                .expect("Invalid value for 'output': {output:?}"),
            "--png",
            "-d",
            "crop=\"#t\"",
            "-d",
            "resolution=\"400\"",
            "-dno-print-pages",
            input
                .canonicalize()?
                .to_str()
                .expect("Invalid value for 'input': {input:?}"),
        ])
        .spawn()
    {
        Ok(child) => match child.wait_with_output() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
