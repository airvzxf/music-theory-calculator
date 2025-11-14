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
 * src/tonic-music/src/lib.rs
 *
 * This is our library crate. All music theory
 * logic will live here.
 */

// 'derive' gives us "free" functionality for this enum.
// Debug:   Lets us print it with println!("{:?}", note);
// Copy/Clone: Lets us easily copy the note (e.g., let note2 = note1;)
// PartialEq/Eq: Lets us compare them (e.g., if note1 == Note::C { ... })
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Note {
    C,
    CSharp, // Represents C# or Db
    D,
    DSharp, // Represents D# or Eb
    E,
    F,
    FSharp, // Represents F# or Gb
    G,
    GSharp, // Represents G# or Ab
    A,
    ASharp, // Represents A# or Bb
    B,
}

// 'impl' lets us add methods to our Note enum
impl Note {
    /// Converts the Note to a number (0-11).
    /// C = 0, C# = 1, D = 2 ... B = 11
    pub fn as_u8(&self) -> u8 {
        match self {
            Note::C => 0,
            Note::CSharp => 1,
            Note::D => 2,
            Note::DSharp => 3,
            Note::E => 4,
            Note::F => 5,
            Note::FSharp => 6,
            Note::G => 7,
            Note::GSharp => 8,
            Note::A => 9,
            Note::ASharp => 10,
            Note::B => 11,
        }
    }

    /// Converts a number (0-11) back to a Note.
    /// This function will panic if the number is > 11.
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => Note::C,
            1 => Note::CSharp,
            2 => Note::D,
            3 => Note::DSharp,
            4 => Note::E,
            5 => Note::F,
            6 => Note::FSharp,
            7 => Note::G,
            8 => Note::GSharp,
            9 => Note::A,
            10 => Note::ASharp,
            11 => Note::B,
            // Use 'panic' for unrecoverable errors.
            // A value > 11 is a logic error in our program.
            _ => panic!("Invalid note value: {}", val),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Interval {
    // We only need intervals up to an Octave for now
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    Tritone,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,
}

impl Interval {
    /// Returns the size of the interval in semitones.
    pub fn as_u8(&self) -> u8 {
        match self {
            Interval::Unison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::Tritone => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::Octave => 12,
        }
    }
}

/// Transposes a root note by a given interval.
///
/// # Examples
///
/// ```
/// // This example won't run as a doc-test because
/// // Note and Interval aren't in scope.
/// // let note = transpose(Note::C, Interval::PerfectFifth);
/// // assert_eq!(note, Note::G);
/// ```
pub fn transpose(root: Note, interval: Interval) -> Note {
    // Get the numeric value of the note and interval
    let root_val = root.as_u8();
    let interval_val = interval.as_u8();

    // Add them together.
    // We use the modulo (%) operator to wrap around the 12 notes.
    // (e.g., A (9) + MajThird (4) = 13.  13 % 12 = 1 (CSharp))
    let new_val = (root_val + interval_val) % 12;

    // Convert the number back to a Note
    Note::from_u8(new_val)
}

/// Represents different types of scales.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScaleType {
    Major,
    MinorNatural,
    MinorHarmonic,
    // We can add more later (MinorMelodic, Pentatonic, etc.)
}

impl ScaleType {
    /// Returns the formula (list of intervals from the root) for a given scale type.
    /// We use a 'static slice' (&'static [Interval]) because these formulas
    /// are fixed and known at compile time. They live for the
    /// entire duration of the program.
    pub fn intervals(&self) -> &'static [Interval] {
        match self {
            ScaleType::Major => &[
                Interval::Unison,       // 1
                Interval::MajorSecond,  // 2
                Interval::MajorThird,   // 3
                Interval::PerfectFourth, // 4
                Interval::PerfectFifth,  // 5
                Interval::MajorSixth,   // 6
                Interval::MajorSeventh, // 7
            ],
            ScaleType::MinorNatural => &[
                Interval::Unison,       // 1
                Interval::MajorSecond,  // 2
                Interval::MinorThird,   // b3
                Interval::PerfectFourth, // 4
                Interval::PerfectFifth,  // 5
                Interval::MinorSixth,   // b6
                Interval::MinorSeventh, // b7
            ],
            ScaleType::MinorHarmonic => &[
                Interval::Unison,       // 1
                Interval::MajorSecond,  // 2
                Interval::MinorThird,   // b3
                Interval::PerfectFourth, // 4
                Interval::PerfectFifth,  // 5
                Interval::MinorSixth,   // b6
                Interval::MajorSeventh, // 7 (The raised 7th)
            ],
        }
    }
}

/// Builds a Vec<Note> for a scale given a root note and scale type.
pub fn build_scale(root: Note, scale_type: ScaleType) -> Vec<Note> {
    // 1. Get the interval formula for the scale
    let intervals = scale_type.intervals();

    // 2. Iterate over the intervals
    // 3. For each interval, transpose the root note
    // 4. Collect the resulting notes into a Vec
    intervals
        .iter()
        .map(|&interval| transpose(root, interval))
        .collect()
}

/// Represents different types of chords (triads for now).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChordType {
    Major,
    Minor,
    Diminished,
    Augmented,
    // We can add 7th chords later
}

impl ChordType {
    /// Returns the formula (list of intervals from the root) for a given chord type.
    pub fn intervals(&self) -> &'static [Interval] {
        match self {
            ChordType::Major => &[
                Interval::Unison,       // 1
                Interval::MajorThird,   // 3
                Interval::PerfectFifth, // 5
            ],
            ChordType::Minor => &[
                Interval::Unison,       // 1
                Interval::MinorThird,   // b3
                Interval::PerfectFifth, // 5
            ],
            ChordType::Diminished => &[
                Interval::Unison,       // 1
                Interval::MinorThird,   // b3
                Interval::Tritone,      // b5
            ],
            ChordType::Augmented => &[
                Interval::Unison,       // 1
                Interval::MajorThird,   // 3
                Interval::MinorSixth,   // #5 (or AugFifth)
            ],
        }
    }

    /// Determines a chord type based on the semitone distance
    /// of its third and fifth from the root.
    pub fn from_intervals(third_interval: u8, fifth_interval: u8) -> Self {
        match (third_interval, fifth_interval) {
            // Major: Major Third (4) + Perfect Fifth (7)
            (4, 7) => ChordType::Major,
            // Minor: Minor Third (3) + Perfect Fifth (7)
            (3, 7) => ChordType::Minor,
            // Diminished: Minor Third (3) + Tritone (6)
            (3, 6) => ChordType::Diminished,
            // Augmented: Major Third (4) + Minor Sixth (8, or AugFifth)
            (4, 8) => ChordType::Augmented,

            // This is a "catch-all"
            // We panic! because our diatonic math should *never*
            // produce a chord we don't recognize.
            // If this code runs, our harmonization logic is wrong.
            _ => panic!(
                "Failed to identify chord from intervals: 3rd={}, 5th={}",
                third_interval, fifth_interval
            ),
        }
    }
}

/// Builds a Vec<Note> for a chord given a root note and chord type.
pub fn build_chord(root: Note, chord_type: ChordType) -> Vec<Note> {
    // 1. Get the interval formula for the chord
    let intervals = chord_type.intervals();

    // 2. Iterate, transpose, and collect
    intervals
        .iter()
        .map(|&interval| transpose(root, interval))
        .collect()
}

/// Represents a single chord in a harmonized scale.
#[derive(Debug, PartialEq, Eq)]
pub struct HarmonizedDegree {
    pub degree: usize, // 1-indexed (I, II, III...)
    pub root_note: Note,
    pub chord_type: ChordType,
    pub notes: Vec<Note>,
}

/// Builds the diatonic triad chords for a given scale.
/// The scale must contain 7 notes.
pub fn harmonize_scale(scale: &[Note]) -> Vec<HarmonizedDegree> {
    // Ensure we have a 7-note scale (heptatonic)
    if scale.len() != 7 {
        // Return an empty vec if not. We could also panic! or return a Result.
        return Vec::new();
    }

    let mut harmonized_scale = Vec::new();

    // Iterate 7 times, once for each degree of the scale
    for i in 0..7 {
        // 1. Get the 1st, 3rd, and 5th notes for this degree
        // We use modular arithmetic to "wrap around" the scale
        let root = scale[i];
        let third = scale[(i + 2) % 7];
        let fifth = scale[(i + 4) % 7];

        // 2. Calculate the intervals *between* these notes
        let root_val = root.as_u8();
        let third_val = third.as_u8();
        let fifth_val = fifth.as_u8();

        // This is the correct, safe way to find the distance:
        // (destination + 12 - origin) % 12
        let third_interval = (third_val + 12 - root_val) % 12;
        let fifth_interval = (fifth_val + 12 - root_val) % 12;

        // 3. Determine the chord type from these intervals
        let chord_type = ChordType::from_intervals(third_interval, fifth_interval);

        // 4. Store the result
        harmonized_scale.push(HarmonizedDegree {
            degree: i + 1, // Store as 1-indexed
            root_note: root,
            chord_type,
            notes: vec![root, third, fifth],
        });
    }

    harmonized_scale
}