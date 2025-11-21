/*
 * tonic-music-cli
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
use crate::Note;

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

#[cfg(test)]
mod tests {
    // We import the parsing functions from the parent module (parser.rs)
    use super::*;
    // We also need the library's enums for comparison.
    use crate::Note;

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
}
