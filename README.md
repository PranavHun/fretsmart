# fretsmart 0.1.0
*A tool to sharpen your fretboard skills on any string instrument*

## Description
Fretsmart is a fretboard visualization tool for stringed instruments. This helps you see (and memorize) the scales, chords or any group of notes as they'd appear on a string instrument's fretboard.

Example: You can choose 'guitar', as the *instrument*, with 'standard' *tuning* and 'D' as the *tuning note*. This prints the fretboard for a guitar with a standard tuning, tuned a whole step lower. You can also provide the *highlight-type* as 'scale', *highlight* as 'mixolydian', for the *highlight-note* 'Bb'. These additional options print the notes of the Bb mixolydian scale on the fretboard with a different color, thus highlighting the selection.

## Features (fully customizable and extendable)
- Instruments - Guitar, Bass, Ukulele, Violin etc.
- Tunings - Standard, DADGAD, OpenG etc.
- Highlight - Scales (Ionian, Blues etc.), Chords (Major, m7, sus4, etc.), any other highlight type
- Create multiple data files to organize highlight groups.

## Usage
```
fretsmart [COMMANDS | SELECTION_OPTIONS] [DATA_FILE]

DATA_FILE                use this instead of the default data file,
                         (~/connfig/fretsmart/data.json)

COMMANDS:
  list [OPTION]          lists entries from DATA_FILE
      instruments
      tunings
      highlights
      datafile           list entire DATA_FILE
  update                 add/modify/delete entries from DATA_FILE

SELECTION_OPTIONS:       default values from DATA_FILE.
  --instrument <name>
  --tuning <name>
  --tuning-note <name>
  --highlight-type <name>
  --highlight <name>
  --highlight-note <name>
```
