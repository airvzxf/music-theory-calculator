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
use tonic_music::Note;

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

#[cfg(test)]
mod tests {
    // We import the parsing functions from the parent module (parser.rs)
    use super::*;
    // We also need the library's enums for comparison.
    use tonic_music::Note;

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
}
