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
 * src/tonic-music/src/cli.rs
 *
 * This module defines the command-line interface
 * structure using clap.
 */

use clap::{Parser, ValueEnum};
use serde::Serialize;
use tonic_music::{ChordType, HarmonicFormula, ScaleType};

/// Available output formats
#[derive(ValueEnum, Clone, Debug, Default, Serialize)]
#[clap(rename_all = "kebab-case")]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Markdown,
}

/// A command-line music theory calculator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The command to run (scale, chord, harmonize)
    #[command(subcommand)]
    pub command: Commands,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text, global = true)]
    pub format: OutputFormat,
}

/// Defines the available subcommands
#[derive(Parser, Debug)]
pub enum Commands {
    /// Generate the notes of a scale
    Scale {
        /// The root note of the scale (e.g., C, CSharp, Db)
        #[arg(short, long)]
        root: String,

        /// The type of scale (e.g., major, minor, harmonic)
        #[arg(short, long, value_enum)]
        scale_type: ScaleType,
    },

    /// Generate the notes of a chord
    Chord {
        /// The root note of the chord
        #[arg(short, long)]
        root: String,

        /// The type of chord (e.g., major, min, dim, aug)
        #[arg(short, long, value_enum)]
        chord_type: ChordType,

        /// Also display the chord's inversions
        #[arg(long)]
        inversions: bool,
    },

    /// Harmonize a scale (find all its diatonic chords)
    Harmonize {
        /// The root note of the scale to harmonize
        #[arg(short, long)]
        root: String,

        /// The type of scale to harmonize
        #[arg(short, long, value_enum)]
        scale_type: ScaleType,

        /// Generate diatonic 7th chords instead of triads
        #[arg(long)]
        sevenths: bool, // `clap` will set it to 'true' if the flag is present, or 'false' if it is not.
    },

    /// Generate a harmonic progression (chord formula)
    Progression {
        /// The root note of the progression (e.g., C, Bb)
        #[arg(short, long)]
        root: String,

        /// The name of the formula (e.g., block, circle)
        #[arg(short, long, value_enum)]
        formula: HarmonicFormula,
    },
}
