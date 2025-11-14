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
        _ => panic!("Invalid chord type: {}", s),
    }
}