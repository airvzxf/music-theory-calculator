/*
 * tonic-music-core
 * Copyright (C) 2025 Israel Alberto Roldan Vega
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * Repository: https://github.com/airvzxf/music-theory-calculator/
 */

/*
 * crates/tonic-music-core/src/parser.rs
 *
 * This module handles parsing user input strings
 * into our library's enums.
 */

// We need to import the types from our library
use crate::{ChordType, Interval, Note};
use clap::ValueEnum;

/// Parses a string into a Note enum. Returns Err if invalid.
pub fn parse_note(s: &str) -> Result<Note, String> {
    match s.to_lowercase().as_str() {
        "c" => Ok(Note::C),
        "c#" | "db" => Ok(Note::CSharp),
        "d" => Ok(Note::D),
        "d#" | "eb" => Ok(Note::DSharp),
        "e" => Ok(Note::E),
        "f" => Ok(Note::F),
        "f#" | "gb" => Ok(Note::FSharp),
        "g" => Ok(Note::G),
        "g#" | "ab" => Ok(Note::GSharp),
        "a" => Ok(Note::A),
        "a#" | "bb" => Ok(Note::ASharp),
        "b" => Ok(Note::B),
        _ => Err(format!("Invalid root note: {}", s)),
    }
}

/// Represents the parsed components of a roman numeral chord symbol.
#[derive(Debug, PartialEq, Eq)]
pub struct ParsedRomanChord {
    pub interval_from_root: Interval,
    pub chord_type: ChordType,
    pub degree: String,
}

/// Parses a roman numeral string (e.g. "IV", "vii", "bVI7") into a chord specification.
pub fn parse_roman_chord(input: &str) -> Result<ParsedRomanChord, String> {
    let mut rest = input;
    let mut accidental_offset = 0; // 0 = none, -1 = flat, 1 = sharp

    // 1. Accidental
    if rest.starts_with('#') {
        accidental_offset = 1;
        rest = &rest[1..];
    } else if rest.starts_with('b') {
        // Ensure "b" is not part of a weird suffix or note name if we were parsing notes,
        // but for Roman numerals, bI, bII etc are valid.
        accidental_offset = -1;
        rest = &rest[1..];
    }

    // 2. Numeral
    // Extract leading I, V chars.
    let numeral_end = rest
        .find(|c: char| !matches!(c, 'i' | 'I' | 'v' | 'V'))
        .unwrap_or(rest.len());
    let numeral_str = &rest[..numeral_end];
    let suffix = &rest[numeral_end..];

    if numeral_str.is_empty() {
        return Err(format!(
            "Invalid roman numeral in '{}': missing numeral",
            input
        ));
    }

    // 3. Decode Numeral (1-7)
    let (base_degree, _is_valid_numeral) = match numeral_str.to_uppercase().as_str() {
        "I" => (1, true),
        "II" => (2, true),
        "III" => (3, true),
        "IV" => (4, true),
        "V" => (5, true),
        "VI" => (6, true),
        "VII" => (7, true),
        _ => return Err(format!("Invalid roman numeral: {}", numeral_str)),
    };

    let is_uppercase = numeral_str.chars().next().unwrap().is_uppercase();

    // 4. Determine Interval
    // Map degree 1-7 to Major Scale intervals (semitones)
    let semitones_maj = match base_degree {
        1 => 0,
        2 => 2,
        3 => 4,
        4 => 5,
        5 => 7,
        6 => 9,
        7 => 11,
        _ => unreachable!(),
    };

    let mut semitones_i8 = semitones_maj as i8 + accidental_offset;
    // Normalize to 0-11
    semitones_i8 = (semitones_i8 + 12) % 12;

    let interval = match semitones_i8 {
        0 => Interval::Unison,
        1 => Interval::MinorSecond,
        2 => Interval::MajorSecond,
        3 => Interval::MinorThird,
        4 => Interval::MajorThird,
        5 => Interval::PerfectFourth,
        6 => Interval::Tritone,
        7 => Interval::PerfectFifth,
        8 => Interval::MinorSixth,
        9 => Interval::MajorSixth,
        10 => Interval::MinorSeventh,
        11 => Interval::MajorSeventh,
        _ => unreachable!(),
    };

    // 5. Determine Chord Type
    let basic_triad = if is_uppercase {
        ChordType::Major
    } else {
        ChordType::Minor
    };

    let chord_type = if suffix.is_empty() {
        basic_triad
    } else {
        match suffix {
            // Context-sensitive "7"
            "7" => {
                if is_uppercase {
                    ChordType::Dominant7
                } else {
                    ChordType::Minor7
                }
            }

            // Explicit overrides
            "maj7" => ChordType::Major7,
            "m7" | "min7" => ChordType::Minor7,
            "dom7" => ChordType::Dominant7,
            "dim" => ChordType::Diminished,
            "dim7" => ChordType::Diminished7,
            "aug" => ChordType::Augmented,
            "m7b5" => ChordType::Minor7b5,

            // Fallback to Clap's parsing (covers aliases like 'mmaj7')
            _ => ChordType::from_str(suffix, true)
                .map_err(|_| format!("Unknown chord suffix: {}", suffix))?,
        }
    };

    Ok(ParsedRomanChord {
        interval_from_root: interval,
        chord_type,
        degree: input.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChordType, Interval, Note};

    #[test]
    fn test_parse_note_simple() {
        assert_eq!(parse_note("C"), Ok(Note::C));
        assert_eq!(parse_note("E"), Ok(Note::E));
    }

    #[test]
    fn test_parse_note_case_insensitive() {
        assert_eq!(parse_note("c"), Ok(Note::C)); // Lower case
        assert_eq!(parse_note("f#"), Ok(Note::FSharp)); // Lowercase sharp
        assert_eq!(parse_note("Db"), Ok(Note::CSharp)); // Capital letter flat
    }

    #[test]
    fn test_parse_note_aliases() {
        assert_eq!(parse_note("C#"), Ok(Note::CSharp));
        assert_eq!(parse_note("Db"), Ok(Note::CSharp)); // Alias Db -> CSharp
        assert_eq!(parse_note("Eb"), Ok(Note::DSharp)); // Alias Eb -> DSharp
        assert_eq!(parse_note("bb"), Ok(Note::ASharp)); // Alias bb -> ASharp
    }

    #[test]
    fn test_parse_note_invalid() {
        assert!(parse_note("H").is_err());
    }

    #[test]
    fn test_parse_roman_simple() {
        // I -> Unison, Major
        let res = parse_roman_chord("I").unwrap();
        assert_eq!(res.interval_from_root, Interval::Unison);
        assert_eq!(res.chord_type, ChordType::Major);

        // iv -> PerfectFourth, Minor
        let res = parse_roman_chord("iv").unwrap();
        assert_eq!(res.interval_from_root, Interval::PerfectFourth);
        assert_eq!(res.chord_type, ChordType::Minor);
    }

    #[test]
    fn test_parse_roman_accidental() {
        // bVII -> MinorSeventh (11-1=10), Major
        let res = parse_roman_chord("bVII").unwrap();
        assert_eq!(res.interval_from_root, Interval::MinorSeventh);
        assert_eq!(res.chord_type, ChordType::Major);

        // #IV -> Tritone (5+1=6), Major
        let res = parse_roman_chord("#IV").unwrap();
        assert_eq!(res.interval_from_root, Interval::Tritone);
        assert_eq!(res.chord_type, ChordType::Major);
    }

    #[test]
    fn test_parse_roman_suffixes() {
        // V7 -> Dominant7
        let res = parse_roman_chord("V7").unwrap();
        assert_eq!(res.chord_type, ChordType::Dominant7);

        // ii7 -> Minor7
        let res = parse_roman_chord("ii7").unwrap();
        assert_eq!(res.chord_type, ChordType::Minor7);

        // Imaj7
        let res = parse_roman_chord("Imaj7").unwrap();
        assert_eq!(res.chord_type, ChordType::Major7);

        // viidim
        let res = parse_roman_chord("viidim").unwrap();
        assert_eq!(res.chord_type, ChordType::Diminished);
    }
}
