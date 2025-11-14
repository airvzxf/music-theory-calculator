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
 * src/tonic-music/src/main.rs
 *
 * This is our binary crate. It will only be
 * responsible for parsing user commands and calling our library.
 */

use clap::Parser;
// Import our library's functions and structs
use tonic_music::{ChordType, build_chord, build_scale, harmonize_scale};

// Declare the new parser.rs module
mod parser;
// Use the public functions from our parser module
use parser::{parse_chord_type, parse_note, parse_scale_type};

// Declare the CLI module
mod cli;
use cli::{Cli, Commands};

fn main() {
    let cli_args = Cli::parse();

    // This match works because the fields in `Commands` are pub
    match &cli_args.command {
        Commands::Scale { root, scale_type } => {
            let root_note = parse_note(root);
            let scale = parse_scale_type(scale_type);

            // Call the library
            let notes = build_scale(root_note, scale);

            println!("--- {} {} Scale ---", root, scale_type);
            println!("{:?}", notes);
        }
        Commands::Chord { root, chord_type } => {
            let root_note = parse_note(root);
            let chord = parse_chord_type(chord_type);

            // Call the library
            let notes = build_chord(root_note, chord);

            println!("--- {} {} Chord ---", root, chord_type);
            println!("{:?}", notes);
        }
        Commands::Harmonize { root, scale_type } => {
            let root_note = parse_note(root);
            let scale = parse_scale_type(scale_type);

            // Call the library
            let scale_notes = build_scale(root_note, scale);
            let harmony = harmonize_scale(&scale_notes);

            println!("--- {} {} Harmonization ---", root, scale_type);

            // Define degree names
            let roman_numerals = ["I", "II", "III", "IV", "V", "VI", "VII"];

            for degree in &harmony {
                let degree_name = roman_numerals[degree.degree - 1];
                let quality = match degree.chord_type {
                    ChordType::Major => "",
                    ChordType::Minor => "m",
                    ChordType::Diminished => "°",
                    ChordType::Augmented => "+",
                };

                // Format: e.g., "I (C): C Major -> [C, E, G]"
                println!(
                    "{} ({:?}):\t{:?} {} \t-> {:?}",
                    degree_name, degree.root_note, degree.root_note, quality, degree.notes
                );
            }
        }
    }
}
