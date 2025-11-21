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
 * crates/tonic-music-cli/src/main.rs
 *
 * This is our binary crate. It will only be
 * responsible for parsing user commands and calling our library.
 */

use clap::Parser;
use serde::Serialize;
// Import our library's functions and structs
use tonic_music_core::{
    ChordType, HarmonizedDegree, Note, ProgressionChord, build_chord, build_custom_progression,
    build_progression, build_scale, get_inversions, harmonize_scale, parser::parse_note,
    parser::parse_roman_chord,
};

// Declare the CLI module
mod cli;
use cli::{Cli, Commands, OutputFormat};

// --- Response Structs for Output Strategy ---

trait Markdown {
    fn to_markdown(&self) -> String;
}

#[derive(Serialize)]
struct ScaleResponse {
    root: String,
    scale_type: String,
    notes: Vec<Note>,
}

impl std::fmt::Display for ScaleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- {} {} Scale ---", self.root, self.scale_type)?;
        write!(f, "{:?}", self.notes)
    }
}

impl Markdown for ScaleResponse {
    fn to_markdown(&self) -> String {
        format!(
            "# {} {} Scale\n\n**Notes:**\n{:?}",
            self.root, self.scale_type, self.notes
        )
    }
}

#[derive(Serialize)]
struct ChordResponse {
    root: String,
    chord_type: String,
    notes: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inversions: Option<Vec<Vec<Note>>>,
}

impl std::fmt::Display for ChordResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- {} {} Chord ---", self.root, self.chord_type)?;

        if let Some(inversions) = &self.inversions {
            let titles = ["Root:", "1st Inv:", "2nd Inv:", "3rd Inv:", "4th Inv:"];
            for (i, inv) in inversions.iter().enumerate() {
                let title = titles.get(i).cloned().unwrap_or("Inv:");
                writeln!(f, "{} \t{:?}", title, inv)?;
            }
            Ok(())
        } else {
            write!(f, "{:?}", self.notes)
        }
    }
}

impl Markdown for ChordResponse {
    fn to_markdown(&self) -> String {
        let mut md = format!("# {} {} Chord\n\n", self.root, self.chord_type);
        if let Some(inversions) = &self.inversions {
            md.push_str("## Inversions\n");
            let titles = ["Root", "1st Inv", "2nd Inv", "3rd Inv", "4th Inv"];
            for (i, inv) in inversions.iter().enumerate() {
                let title = titles.get(i).cloned().unwrap_or("Inv");
                md.push_str(&format!("- **{}:** {:?}\n", title, inv));
            }
        } else {
            md.push_str(&format!("**Notes:** {:?}\n", self.notes));
        }
        md
    }
}

#[derive(Serialize)]
struct HarmonizeResponse {
    root: String,
    scale_type: String,
    harmony: Vec<HarmonizedDegree>,
}

impl std::fmt::Display for HarmonizeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- {} {} Harmonization ---", self.root, self.scale_type)?;

        let roman_numerals = ["I", "II", "III", "IV", "V", "VI", "VII"];

        for degree in &self.harmony {
            let degree_name = roman_numerals.get(degree.degree - 1).unwrap_or(&"?");
            let quality = get_chord_quality_symbol(degree.chord_type);

            writeln!(
                f,
                "{} ({:?}):\t{:?} {} \t-> {:?}",
                degree_name, degree.root_note, degree.root_note, quality, degree.notes
            )?;
        }
        Ok(())
    }
}

impl Markdown for HarmonizeResponse {
    fn to_markdown(&self) -> String {
        let mut md = format!(
            "# {} {} Harmonization\n\n| Degree | Note | Chord | Notes |\n|---|---|---|---|\n",
            self.root, self.scale_type
        );
        let roman_numerals = ["I", "II", "III", "IV", "V", "VI", "VII"];
        for degree in &self.harmony {
            let degree_name = roman_numerals.get(degree.degree - 1).unwrap_or(&"?");
            let quality = get_chord_quality_symbol(degree.chord_type);
            md.push_str(&format!(
                "| {} | {:?} | {:?}{} | {:?} |\n",
                degree_name, degree.root_note, degree.root_note, quality, degree.notes
            ));
        }
        md
    }
}

#[derive(Serialize)]
struct ProgressionResponse {
    root: String,
    formula: String,
    progression: Vec<ProgressionChord>,
}

impl std::fmt::Display for ProgressionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- {} {} Progression ---", self.root, self.formula)?;
        for chord in &self.progression {
            let quality = get_chord_quality_symbol(chord.chord_type);
            writeln!(
                f,
                "{}:\t{:?} {} \t-> {:?}",
                chord.degree, chord.root_note, quality, chord.notes
            )?;
        }
        Ok(())
    }
}

impl Markdown for ProgressionResponse {
    fn to_markdown(&self) -> String {
        let mut md = format!(
            "# {} {} Progression\n\n| Degree | Chord | Notes |\n|---|---|---|\n",
            self.root, self.formula
        );
        for chord in &self.progression {
            let quality = get_chord_quality_symbol(chord.chord_type);
            md.push_str(&format!(
                "| {} | {:?}{} | {:?} |\n",
                chord.degree, chord.root_note, quality, chord.notes
            ));
        }
        md
    }
}

fn get_chord_quality_symbol(chord_type: ChordType) -> &'static str {
    match chord_type {
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
    }
}

fn print_output<T: Serialize + std::fmt::Display + Markdown>(data: &T, format: OutputFormat) {
    match format {
        OutputFormat::Text => print!("{}", data),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(data).unwrap()),
        OutputFormat::Markdown => println!("{}", data.to_markdown()),
    }
}

fn main() {
    let cli_args = Cli::parse();

    match &cli_args.command {
        Commands::Scale { root, scale_type } => {
            let root_note = parse_note(root).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            // scale_type is already typed
            let notes = build_scale(root_note, *scale_type);

            let response = ScaleResponse {
                root: root.clone(),
                scale_type: format!("{:?}", scale_type),
                notes,
            };
            print_output(&response, cli_args.format);
        }
        Commands::Chord {
            root,
            chord_type,
            inversions,
        } => {
            let root_note = parse_note(root).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            let notes = build_chord(root_note, *chord_type);

            let invs = if *inversions {
                Some(get_inversions(&notes))
            } else {
                None
            };

            let response = ChordResponse {
                root: root.clone(),
                chord_type: format!("{:?}", chord_type),
                notes,
                inversions: invs,
            };
            print_output(&response, cli_args.format);
        }
        Commands::Harmonize {
            root,
            scale_type,
            sevenths,
        } => {
            let root_note = parse_note(root).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            let scale_notes = build_scale(root_note, *scale_type);
            let harmony = harmonize_scale(&scale_notes, *sevenths);

            let response = HarmonizeResponse {
                root: root.clone(),
                scale_type: format!("{:?}", scale_type),
                harmony,
            };
            print_output(&response, cli_args.format);
        }
        Commands::Progression {
            root,
            formula,
            custom,
        } => {
            let root_note = parse_note(root).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });

            let (progression, formula_name) = if let Some(f) = formula {
                (build_progression(root_note, *f), format!("{:?}", f))
            } else if let Some(c) = custom {
                // Split by '-' or space
                let parts: Vec<&str> = c.split(&['-', ' '][..]).filter(|s| !s.is_empty()).collect();

                let specs_res: Result<Vec<_>, _> =
                    parts.into_iter().map(parse_roman_chord).collect();

                let specs = specs_res.unwrap_or_else(|e| {
                    eprintln!("Error parsing custom progression: {}", e);
                    std::process::exit(1);
                });

                (build_custom_progression(root_note, specs), c.clone())
            } else {
                unreachable!("Clap ensures one is present");
            };

            let response = ProgressionResponse {
                root: root.clone(),
                formula: formula_name,
                progression,
            };
            print_output(&response, cli_args.format);
        }
    }
}
