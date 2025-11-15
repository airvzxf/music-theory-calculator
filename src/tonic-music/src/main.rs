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
use tonic_music::{ChordType, build_chord, build_scale, get_inversions, harmonize_scale};

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
        Commands::Chord {
            root,
            chord_type,
            inversions,
        } => {
            let root_note = parse_note(root);
            let chord = parse_chord_type(chord_type);

            // Call the library
            let notes = build_chord(root_note, chord);

            println!("--- {} {} Chord ---", root, chord_type);

            if *inversions {
                // Call the library's new feature
                let all_inversions = get_inversions(&notes);
                let titles = ["Root:", "1st Inv:", "2nd Inv:", "3rd Inv:", "4th Inv:"];

                for (i, inv) in all_inversions.iter().enumerate() {
                    // Use the title or a generic term if there are more inversions than expected.
                    let title = titles.get(i).cloned().unwrap_or("Inv:");
                    println!("{} \t{:?}", title, inv);
                }
            } else {
                // Previous behavior
                println!("{:?}", notes);
            }
        }
        Commands::Harmonize {
            root,
            scale_type,
            sevenths,
        } => {
            let root_note = parse_note(root);
            let scale = parse_scale_type(scale_type);

            // Call the library
            let scale_notes = build_scale(root_note, scale);
            let harmony = harmonize_scale(&scale_notes, *sevenths);

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
                    ChordType::Major7 => "maj7",
                    ChordType::Minor7 => "m7",
                    ChordType::Dominant7 => "7",
                    ChordType::Minor7b5 => "m7b5",
                    ChordType::Diminished7 => "°7",
                    ChordType::MinorMajor7 => "m(maj7)",
                    ChordType::AugmentedMajor7 => "aug(maj7)",
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
