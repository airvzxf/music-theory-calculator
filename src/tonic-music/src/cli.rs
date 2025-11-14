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

use clap::Parser;

/// A command-line music theory calculator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The command to run (scale, chord, harmonize)
    #[command(subcommand)]
    pub command: Commands,
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
        #[arg(short, long)]
        scale_type: String,
    },

    /// Generate the notes of a chord
    Chord {
        /// The root note of the chord
        #[arg(short, long)]
        root: String,

        /// The type of chord (e.g., major, min, dim, aug)
        #[arg(short, long)]
        chord_type: String,
    },

    /// Harmonize a scale (find all its diatonic chords)
    Harmonize {
        /// The root note of the scale to harmonize
        #[arg(short, long)]
        root: String,

        /// The type of scale to harmonize
        #[arg(short, long)]
        scale_type: String,
    },
}
