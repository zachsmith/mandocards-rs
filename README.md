Creates an Anki deck for Mandolin notes and fret positions.

Requires `lilypond` application to be installed on the system. Needed for notation and fretboard generation.
Does not require `Anki` application to be installed for building, but does for viewing.

Has three sub-commands:

**generate**: will create the Lilypond score file: `output/lilypond/mandocard.ly`

**compile**: (currently does not work; instead run the lilypond command directly from within the `output/lilypond` folder) ` lilypond --png -d crop="#t" -d resolution="400" -dno-print-pages --output ../anki/mandocard.ly`

**build**: will generate the anki deck file which can be imported into `Anki`: `output/anki/mandocard.apkg`