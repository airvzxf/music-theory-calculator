/*
 * tonic-music
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
 * src/tonic-music/src/parser.rs
 *
 * This module handles parsing user input strings
 * into our library's enums.
 */

// We need to import the types from our library
use tonic_music::{ChordType, Note, ScaleType};

/// Parses a string into a Note enum. Panics if invalid.
pub fn parse_note(s: &str) -> Note {
    match s.to_lowercase().as_str() {
        "c" => Note::C,
        "c#" | "db" => Note::CSharp,
        "d" => Note::D,
        "d#" | "eb" => Note::DSharp,
        "e" => Note::E,
        "f" => Note::F,
        "f#" | "gb" => Note::FSharp,
        "g" => Note::G,
        "g#" | "ab" => Note::GSharp,
        "a" => Note::A,
        "a#" | "bb" => Note::ASharp,
        "b" => Note::B,
        _ => panic!("Invalid root note: {}", s),
    }
}

/// Parses a string into a ScaleType enum. Panics if invalid.
pub fn parse_scale_type(s: &str) -> ScaleType {
    match s.to_lowercase().as_str() {
        "major" | "maj" => ScaleType::Major,
        "minor" | "natural" | "minor-natural" => ScaleType::MinorNatural,
        "harmonic" | "minor-harmonic" => ScaleType::MinorHarmonic,
        "penta-major" | "pentatonic-major" => ScaleType::PentatonicMajor,
        "penta-minor" | "pentatonic-minor" => ScaleType::PentatonicMinor,
        _ => panic!("Invalid scale type: {}", s),
    }
}

/// Parses a string into a ChordType enum. Panics if invalid.
pub fn parse_chord_type(s: &str) -> ChordType {
    match s.to_lowercase().as_str() {
        "major" | "maj" => ChordType::Major,
        "minor" | "min" => ChordType::Minor,
        "diminished" | "dim" => ChordType::Diminished,
        "augmented" | "aug" => ChordType::Augmented,
        "major7" | "maj7" => ChordType::Major7,
        "minor7" | "min7" | "m7" => ChordType::Minor7,
        "dominant7" | "dom7" | "7" => ChordType::Dominant7,
        "minor7b5" | "m7b5" | "half-diminished" => ChordType::Minor7b5,
        "diminished7" | "dim7" => ChordType::Diminished7,
        "minormajor7" | "mmaj7" | "m(maj7)" => ChordType::MinorMajor7,
        "augmentedmajor7" | "augmaj7" | "aug(maj7)" => ChordType::AugmentedMajor7,
        _ => panic!("Invalid chord type: {}", s),
    }
}

#[cfg(test)]
mod tests {
    // We import the parsing functions from the parent module (parser.rs)
    use super::*;
    // We also need the library's enums for comparison.
    use tonic_music::{ChordType, Note, ScaleType};

    #[test]
    fn test_parse_note_simple() {
        assert_eq!(parse_note("C"), Note::C);
        assert_eq!(parse_note("E"), Note::E);
    }

    #[test]
    fn test_parse_note_case_insensitive() {
        assert_eq!(parse_note("c"), Note::C); // Lower case
        assert_eq!(parse_note("f#"), Note::FSharp); // Lowercase sharp
        assert_eq!(parse_note("Db"), Note::CSharp); // Capital letter flat
    }

    #[test]
    fn test_parse_note_aliases() {
        assert_eq!(parse_note("C#"), Note::CSharp);
        assert_eq!(parse_note("Db"), Note::CSharp); // Alias Db -> CSharp
        assert_eq!(parse_note("Eb"), Note::DSharp); // Alias Eb -> DSharp
        assert_eq!(parse_note("bb"), Note::ASharp); // Alias bb -> ASharp
    }

    #[test]
    #[should_panic]
    fn test_parse_note_invalid() {
        // This test *passes* if the code panics.
        // which is what we want.
        parse_note("H");
    }

    #[test]
    fn test_parse_scale_type() {
        assert_eq!(parse_scale_type("major"), ScaleType::Major);
        assert_eq!(parse_scale_type("MAJOR"), ScaleType::Major); // Capital letters
        assert_eq!(parse_scale_type("minor"), ScaleType::MinorNatural);
        assert_eq!(parse_scale_type("natural"), ScaleType::MinorNatural); // Alias
        assert_eq!(parse_scale_type("harmonic"), ScaleType::MinorHarmonic);
    }

    #[test]
    #[should_panic]
    fn test_parse_scale_type_invalid() {
        parse_scale_type("phrygian"); // We haven't implemented it yet.
    }

    #[test]
    fn test_parse_chord_type() {
        assert_eq!(parse_chord_type("min"), ChordType::Minor); // Alias
        assert_eq!(parse_chord_type("MAJOR"), ChordType::Major); // Capital letters
        assert_eq!(parse_chord_type("dim"), ChordType::Diminished); // Alias
        assert_eq!(parse_chord_type("augmented"), ChordType::Augmented);
    }

    #[test]
    #[should_panic]
    fn test_parse_chord_type_invalid() {
        // We test with "maj9" now, since "maj7" is valid.
        parse_chord_type("maj9");
    }

    #[test]
    fn test_parse_chord_type_sevenths() {
        assert_eq!(parse_chord_type("maj7"), ChordType::Major7);
        assert_eq!(parse_chord_type("m7"), ChordType::Minor7);
        assert_eq!(parse_chord_type("7"), ChordType::Dominant7);
        assert_eq!(parse_chord_type("m7b5"), ChordType::Minor7b5);
        assert_eq!(parse_chord_type("dim7"), ChordType::Diminished7);
    }

    #[test]
    fn test_parse_scale_type_pentatonic() {
        assert_eq!(parse_scale_type("penta-major"), ScaleType::PentatonicMajor);
        assert_eq!(
            parse_scale_type("pentatonic-minor"),
            ScaleType::PentatonicMinor
        );
    }
}
