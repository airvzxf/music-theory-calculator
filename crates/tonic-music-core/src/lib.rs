/*
 * tonic-music-core
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
 * crates/tonic-music-core/src/lib.rs
 *
 * This is our library crate. All music theory
 * logic will live here.
 */

use clap::ValueEnum;
use serde::Serialize;

pub mod parser;

// 'derive' gives us "free" functionality for this enum.
// Copy/Clone: Lets us easily copy the note (e.g., let note2 = note1;)
// PartialEq/Eq: Lets us compare them (e.g., if note1 == Note::C { ... })
#[derive(Copy, Clone, PartialEq, Eq, Serialize)]
pub enum Note {
    C,
    #[serde(rename = "C#")]
    CSharp, // Represents C# or Db
    D,
    #[serde(rename = "D#")]
    DSharp, // Represents D# or Eb
    E,
    F,
    #[serde(rename = "F#")]
    FSharp, // Represents F# or Gb
    G,
    #[serde(rename = "G#")]
    GSharp, // Represents G# or Ab
    A,
    #[serde(rename = "A#")]
    ASharp, // Represents A# or Bb
    B,
}

impl std::fmt::Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B",
        };
        write!(f, "{}", s)
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
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
/// ```text
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum, Serialize)]
#[clap(rename_all = "kebab-case")]
pub enum ScaleType {
    #[value(alias("maj"))]
    Major,
    #[value(alias("minor"), alias("natural"))]
    MinorNatural,
    #[value(alias("harmonic"))]
    MinorHarmonic,
    #[value(alias("penta-major"))]
    PentatonicMajor,
    #[value(alias("penta-minor"))]
    PentatonicMinor,
}

impl ScaleType {
    /// Returns the formula (list of intervals from the root) for a given scale type.
    /// We use a 'static slice' (&'static [Interval]) because these formulas
    /// are fixed and known at compile time. They live for the
    /// entire duration of the program.
    pub fn intervals(&self) -> &'static [Interval] {
        match self {
            ScaleType::Major => &[
                Interval::Unison,        // 1
                Interval::MajorSecond,   // 2
                Interval::MajorThird,    // 3
                Interval::PerfectFourth, // 4
                Interval::PerfectFifth,  // 5
                Interval::MajorSixth,    // 6
                Interval::MajorSeventh,  // 7
            ],
            ScaleType::MinorNatural => &[
                Interval::Unison,        // 1
                Interval::MajorSecond,   // 2
                Interval::MinorThird,    // b3
                Interval::PerfectFourth, // 4
                Interval::PerfectFifth,  // 5
                Interval::MinorSixth,    // b6
                Interval::MinorSeventh,  // b7
            ],
            ScaleType::MinorHarmonic => &[
                Interval::Unison,        // 1
                Interval::MajorSecond,   // 2
                Interval::MinorThird,    // b3
                Interval::PerfectFourth, // 4
                Interval::PerfectFifth,  // 5
                Interval::MinorSixth,    // b6
                Interval::MajorSeventh,  // 7 (The raised 7th)
            ],
            ScaleType::PentatonicMajor => &[
                Interval::Unison,       // 1
                Interval::MajorSecond,  // 2
                Interval::MajorThird,   // 3
                Interval::PerfectFifth, // 5
                Interval::MajorSixth,   // 6
            ],
            ScaleType::PentatonicMinor => &[
                Interval::Unison,        // 1
                Interval::MinorThird,    // b3
                Interval::PerfectFourth, // 4
                Interval::PerfectFifth,  // 5
                Interval::MinorSeventh,  // b7
            ],
        }
    }
}

/// Builds a `Vec<Note>` for a scale given a root note and scale type.
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum, Serialize)]
#[clap(rename_all = "kebab-case")]
pub enum ChordType {
    #[value(alias("maj"))]
    Major,
    #[value(alias("min"))]
    Minor,
    #[value(alias("dim"))]
    Diminished,
    #[value(alias("aug"))]
    Augmented,
    #[value(alias("maj7"))]
    Major7,
    #[value(alias("min7"), alias("m7"))]
    Minor7,
    #[value(alias("dom7"), alias("7"))]
    Dominant7,
    #[value(alias("m7b5"), alias("half-diminished"))]
    Minor7b5, // Also known as Half-Diminished
    #[value(alias("dim7"))]
    Diminished7, // Also known as Fully-Diminished
    #[value(alias("mmaj7"), alias("m(maj7)"))]
    MinorMajor7, // From the Harmonic Minor scale
    #[value(alias("augmaj7"), alias("aug(maj7)"))]
    AugmentedMajor7, // From the Harmonic Minor scale
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
                Interval::Unison,     // 1
                Interval::MinorThird, // b3
                Interval::Tritone,    // b5
            ],
            ChordType::Augmented => &[
                Interval::Unison,     // 1
                Interval::MajorThird, // 3
                Interval::MinorSixth, // #5 (or AugFifth)
            ],
            ChordType::Major7 => &[
                Interval::Unison,       // 1
                Interval::MajorThird,   // 3
                Interval::PerfectFifth, // 5
                Interval::MajorSeventh, // 7
            ],
            ChordType::Minor7 => &[
                Interval::Unison,       // 1
                Interval::MinorThird,   // b3
                Interval::PerfectFifth, // 5
                Interval::MinorSeventh, // b7
            ],
            ChordType::Dominant7 => &[
                Interval::Unison,       // 1
                Interval::MajorThird,   // 3
                Interval::PerfectFifth, // 5
                Interval::MinorSeventh, // b7
            ],
            ChordType::Minor7b5 => &[
                Interval::Unison,       // 1
                Interval::MinorThird,   // b3
                Interval::Tritone,      // b5
                Interval::MinorSeventh, // b7
            ],
            ChordType::Diminished7 => &[
                Interval::Unison,     // 1
                Interval::MinorThird, // b3
                Interval::Tritone,    // b5
                Interval::MinorSixth, // bb7 (o 6)
            ],
            ChordType::MinorMajor7 => &[
                Interval::Unison,       // 1
                Interval::MinorThird,   // b3
                Interval::PerfectFifth, // 5
                Interval::MajorSeventh, // 7
            ],
            ChordType::AugmentedMajor7 => &[
                Interval::Unison,       // 1
                Interval::MajorThird,   // 3
                Interval::MinorSixth,   // #5
                Interval::MajorSeventh, // 7
            ],
        }
    }

    /// Determines a chord type based on the semitone distance
    /// of its third, fifth, and (optional) seventh from the root.
    pub fn from_intervals(third: u8, fifth: u8, seventh: Option<u8>) -> Self {
        match (third, fifth, seventh) {
            // --- Seventh Cases ---
            (4, 7, Some(11)) => ChordType::Major7,
            (3, 7, Some(10)) => ChordType::Minor7,
            (4, 7, Some(10)) => ChordType::Dominant7,
            (3, 6, Some(10)) => ChordType::Minor7b5,
            (3, 6, Some(9)) => ChordType::Diminished7, // 9 semitones = bb7
            (3, 7, Some(11)) => ChordType::MinorMajor7,
            (4, 8, Some(11)) => ChordType::AugmentedMajor7,

            // --- Triad cases (if seventh is None) ---
            (4, 7, None) => ChordType::Major,
            (3, 7, None) => ChordType::Minor,
            (3, 6, None) => ChordType::Diminished,
            (4, 8, None) => ChordType::Augmented,

            // Panic if it's an unknown combination
            _ => panic!(
                "Failed to identify chord from intervals: 3rd={}, 5th={}, 7th={:?}",
                third, fifth, seventh
            ),
        }
    }
}

/// Builds a `Vec<Note>` for a chord given a root note and chord type.
pub fn build_chord(root: Note, chord_type: ChordType) -> Vec<Note> {
    // 1. Get the interval formula for the chord
    let intervals = chord_type.intervals();

    // 2. Iterate, transpose, and collect
    intervals
        .iter()
        .map(|&interval| transpose(root, interval))
        .collect()
}

/// Calculates all inversions for a given set of chord notes.
/// The first Vec in the list is always the root position.
pub fn get_inversions(chord_notes: &[Note]) -> Vec<Vec<Note>> {
    // If there are no notes, it returns an empty list.
    if chord_notes.is_empty() {
        return Vec::new();
    }

    let mut inversions = Vec::new();
    let mut current_inversion = chord_notes.to_vec();

    // Iterate once for each note in the chord
    for _ in 0..current_inversion.len() {
        // Add the current version to our list
        inversions.push(current_inversion.clone());

        // Turn Vec 1 position to the left
        // [C, E, G] -> [E, G, C]
        current_inversion.rotate_left(1);
    }

    inversions
}

/// Calculates the shortest distance in semitones between two notes.
fn semitone_distance(note1: Note, note2: Note) -> u8 {
    let val1 = note1.as_u8();
    let val2 = note2.as_u8();
    let diff = (val1 as i8 - val2 as i8).unsigned_abs();
    diff.min(12 - diff)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum, Serialize)]
#[clap(rename_all = "kebab-case")]
pub enum HarmonicFormula {
    /// A 12-bar blues-style block progression
    /// Formula: I-Maj, V-Dom7, I-Dom7, IV-Maj
    #[value(alias("blues"))]
    Block,
    /// A diatonic I-vi-ii-V7 "Circle" progression
    Circle,
    /// A I-IV-V7 progression common in Guajira music
    Guajira,
    /// A relative minor block: vi-IV7-ii-III7
    #[value(alias("bloque-rm"))]
    BloqueRm,
}

/// Represents a single chord within a progression
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct ProgressionChord {
    /// The roman numeral degree (e.g., "I", "V7", "I7")
    pub degree: String,
    /// The specific chord (e.g., "Cmaj", "G7")
    pub root_note: Note,
    pub chord_type: ChordType,
    /// The notes of the chord
    pub notes: Vec<Note>,
}

/// Represents a single chord in a harmonized scale.
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct HarmonizedDegree {
    pub degree: usize, // 1-indexed (I, II, III...)
    pub root_note: Note,
    pub chord_type: ChordType,
    pub notes: Vec<Note>,
}

/// Builds the diatonic triad chords for a given scale.
/// The scale must contain 7 notes.
pub fn harmonize_scale(scale: &[Note], build_sevenths: bool) -> Vec<HarmonizedDegree> {
    if scale.len() != 7 {
        return Vec::new();
    }

    let mut harmonized_scale = Vec::new();

    for i in 0..7 {
        // 1. Get the notes for this degree
        let root = scale[i];
        let third = scale[(i + 2) % 7];
        let fifth = scale[(i + 4) % 7];

        // 2. Calculate the intervals
        let root_val = root.as_u8();
        let third_interval = (third.as_u8() + 12 - root_val) % 12;
        let fifth_interval = (fifth.as_u8() + 12 - root_val) % 12;

        let mut notes = vec![root, third, fifth];
        let mut seventh_interval: Option<u8> = None;

        // If the user wants 7 more, calculate the 7th
        if build_sevenths {
            let seventh = scale[(i + 6) % 7];
            let seventh_val = seventh.as_u8();
            seventh_interval = Some((seventh_val + 12 - root_val) % 12);
            notes.push(seventh);
        }

        // 3. Determine the chord type from these intervals
        let chord_type = ChordType::from_intervals(
            third_interval,
            fifth_interval,
            seventh_interval, // Pass Some(val) or None
        );

        // 4. Store the result
        harmonized_scale.push(HarmonizedDegree {
            degree: i + 1,
            root_note: root,
            chord_type,
            notes, // notes now has 3 or 4 notes
        });
    }

    harmonized_scale
}

use parser::ParsedRomanChord;

/// Builds a chord progression from a root note and a formula.
pub fn build_progression(root: Note, formula: HarmonicFormula) -> Vec<ProgressionChord> {
    // 1. Get the sequence of (degree, root, type) from the formula-specific function
    let chord_specs = match formula {
        HarmonicFormula::Block => get_block_progression_spec(root),
        HarmonicFormula::Circle => get_circle_progression_spec(root),
        HarmonicFormula::Guajira => get_guajira_progression_spec(root),
        HarmonicFormula::BloqueRm => get_bloque_rm_progression_spec(root),
    };

    solve_voice_leading(root, chord_specs)
}

/// Builds a custom chord progression from a root note and a list of parsed chord specs.
pub fn build_custom_progression(root: Note, specs: Vec<ParsedRomanChord>) -> Vec<ProgressionChord> {
    let chord_specs = specs
        .into_iter()
        .map(|spec| {
            (
                spec.degree,
                transpose(root, spec.interval_from_root),
                spec.chord_type,
            )
        })
        .collect();

    solve_voice_leading(root, chord_specs)
}

/// Helper to apply voice leading logic to a sequence of chords.
fn solve_voice_leading(
    key_center: Note,
    chord_specs: Vec<(String, Note, ChordType)>,
) -> Vec<ProgressionChord> {
    let mut progression = Vec::new();
    let mut previous_bass_note = key_center;

    // 2. Loop through ALL chords and apply voice leading
    // Note: We include the first chord in this logic so it can start in an inversion
    // if that's closer to the "previous bass" (which starts as the Key Tonic).
    for (degree, chord_root, chord_type) in chord_specs.iter() {
        let root_chord = build_chord(*chord_root, *chord_type);
        let inversions = get_inversions(&root_chord);

        let best_inversion = inversions
            .into_iter()
            .min_by(|inv_a, inv_b| {
                let bass_a = inv_a.first().unwrap();
                let bass_b = inv_b.first().unwrap();

                let dist_a_root = semitone_distance(*bass_a, key_center);
                let dist_b_root = semitone_distance(*bass_b, key_center);

                let dist_a_prev = semitone_distance(*bass_a, previous_bass_note);
                let dist_b_prev = semitone_distance(*bass_b, previous_bass_note);

                // PRIORITIZE SMOOTHNESS (dist to previous bass) over CENTERING (dist to tonic)
                // This creates better voice leading lines (e.g., walking bass).
                (dist_a_prev, dist_a_root).cmp(&(dist_b_prev, dist_b_root))
            })
            .unwrap_or(root_chord); // Fallback

        previous_bass_note = *best_inversion.first().unwrap();

        progression.push(ProgressionChord {
            degree: degree.clone(),
            root_note: *chord_root,
            chord_type: *chord_type,
            notes: best_inversion,
        });
    }

    progression
}

/// Returns the chord specifications for the I-V7-I7-IV "Block" (Blues) progression.
fn get_block_progression_spec(root: Note) -> Vec<(String, Note, ChordType)> {
    vec![
        ("I".to_string(), root, ChordType::Major),
        (
            "V7".to_string(),
            transpose(root, Interval::PerfectFifth),
            ChordType::Dominant7,
        ),
        ("I7".to_string(), root, ChordType::Dominant7),
        (
            "IV".to_string(),
            transpose(root, Interval::PerfectFourth),
            ChordType::Major,
        ),
    ]
}

/// Returns the chord specifications for the I-vi-ii-V7 "Circle" progression.
fn get_circle_progression_spec(root: Note) -> Vec<(String, Note, ChordType)> {
    vec![
        ("I".to_string(), root, ChordType::Major),
        (
            "vi".to_string(),
            transpose(root, Interval::MajorSixth),
            ChordType::Minor,
        ),
        (
            "ii".to_string(),
            transpose(root, Interval::MajorSecond),
            ChordType::Minor,
        ),
        (
            "V7".to_string(),
            transpose(root, Interval::PerfectFifth),
            ChordType::Dominant7,
        ),
    ]
}

/// Returns the chord specifications for the I-IV-V7 "Guajira" progression.
fn get_guajira_progression_spec(root: Note) -> Vec<(String, Note, ChordType)> {
    vec![
        ("I".to_string(), root, ChordType::Major),
        (
            "IV".to_string(),
            transpose(root, Interval::PerfectFourth),
            ChordType::Major,
        ),
        (
            "V7".to_string(),
            transpose(root, Interval::PerfectFifth),
            ChordType::Dominant7,
        ),
    ]
}

/// Returns the chord specifications for the vi-IV7-ii-III7 "Bloque R.m." progression.
fn get_bloque_rm_progression_spec(root: Note) -> Vec<(String, Note, ChordType)> {
    vec![
        (
            "vi".to_string(),
            transpose(root, Interval::MajorSixth),
            ChordType::Minor,
        ),
        (
            "IV7".to_string(),
            transpose(root, Interval::PerfectFourth),
            ChordType::Dominant7,
        ),
        (
            "ii".to_string(),
            transpose(root, Interval::MajorSecond),
            ChordType::Minor,
        ),
        (
            "III7".to_string(),
            transpose(root, Interval::MajorThird),
            ChordType::Dominant7,
        ),
    ]
}

#[cfg(test)]
mod tests {
    // We import everything from the parent module (our lib.rs)
    use super::*;

    #[test]
    fn test_lib_transpose_simple() {
        assert_eq!(transpose(Note::C, Interval::PerfectFifth), Note::G);
    }

    #[test]
    fn test_lib_transpose_wrap_around() {
        // A (9) + MajorThird (4) = 13. 13 % 12 = 1 (CSharp)
        assert_eq!(transpose(Note::A, Interval::MajorThird), Note::CSharp);
    }

    #[test]
    fn test_lib_note_display() {
        assert_eq!(format!("{}", Note::C), "C");
        assert_eq!(format!("{}", Note::CSharp), "C#");
        assert_eq!(format!("{}", Note::FSharp), "F#");
        assert_eq!(format!("{}", Note::B), "B");
    }

    #[test]
    fn test_lib_build_scale_c_major() {
        let scale = build_scale(Note::C, ScaleType::Major);
        let expected = vec![
            Note::C,
            Note::D,
            Note::E,
            Note::F,
            Note::G,
            Note::A,
            Note::B,
        ];
        assert_eq!(scale, expected);
    }

    #[test]
    fn test_lib_build_scale_a_harmonic_minor() {
        let scale = build_scale(Note::A, ScaleType::MinorHarmonic);
        let expected = vec![
            Note::A,
            Note::B,
            Note::C,
            Note::D,
            Note::E,
            Note::F,
            Note::GSharp,
        ];
        assert_eq!(scale, expected);
    }

    #[test]
    fn test_lib_build_chord_a_minor() {
        let chord = build_chord(Note::A, ChordType::Minor);
        let expected = vec![Note::A, Note::C, Note::E];
        assert_eq!(chord, expected);
    }

    #[test]
    fn test_lib_build_chord_b_diminished() {
        let chord = build_chord(Note::B, ChordType::Diminished);
        let expected = vec![Note::B, Note::D, Note::F];
        assert_eq!(chord, expected);
    }

    #[test]
    fn test_lib_chord_type_from_intervals_logic() {
        // Try the logic that gave us problems (Minor)
        // A (9) -> C (0) = 3 semitones
        // A (9) -> E (4) = 7 semitones
        let third_interval = (Note::C.as_u8() + 12 - Note::A.as_u8()) % 12;
        let fifth_interval = (Note::E.as_u8() + 12 - Note::A.as_u8()) % 12;
        assert_eq!(third_interval, 3);
        assert_eq!(fifth_interval, 7);
        assert_eq!(
            ChordType::from_intervals(third_interval, fifth_interval, None),
            ChordType::Minor
        );

        // Test the logic (Diminished)
        // B (11) -> D (2) = 3 semitones
        // B (11) -> F (5) = 6 semitones
        let third_interval_b = (Note::D.as_u8() + 12 - Note::B.as_u8()) % 12;
        let fifth_interval_b = (Note::F.as_u8() + 12 - Note::B.as_u8()) % 12;
        assert_eq!(third_interval_b, 3);
        assert_eq!(fifth_interval_b, 6);
        assert_eq!(
            ChordType::from_intervals(third_interval_b, fifth_interval_b, None),
            ChordType::Diminished
        );
    }

    #[test]
    fn test_lib_harmonize_c_major() {
        let scale = build_scale(Note::C, ScaleType::Major);
        let harmony = harmonize_scale(&scale, false);

        // We extract only the qualities of the chords
        let qualities: Vec<ChordType> = harmony.iter().map(|d| d.chord_type).collect();

        let expected_qualities = vec![
            ChordType::Major,
            ChordType::Minor,
            ChordType::Minor,
            ChordType::Major,
            ChordType::Major,
            ChordType::Minor,
            ChordType::Diminished,
        ];

        assert_eq!(qualities, expected_qualities);
    }

    #[test]
    fn test_lib_harmonize_c_harmonic_minor() {
        let scale = build_scale(Note::C, ScaleType::MinorHarmonic);
        let harmony = harmonize_scale(&scale, false);

        let qualities: Vec<ChordType> = harmony.iter().map(|d| d.chord_type).collect();

        let expected_qualities = vec![
            ChordType::Minor,
            ChordType::Diminished,
            ChordType::Augmented,
            ChordType::Minor,
            ChordType::Major,
            ChordType::Major,
            ChordType::Diminished,
        ];

        assert_eq!(qualities, expected_qualities);
    }

    #[test]
    fn test_lib_build_chord_sevenths() {
        let chord = build_chord(Note::C, ChordType::Major7);
        let expected = vec![Note::C, Note::E, Note::G, Note::B];
        assert_eq!(chord, expected);

        let chord = build_chord(Note::G, ChordType::Dominant7);
        let expected = vec![Note::G, Note::B, Note::D, Note::F];
        assert_eq!(chord, expected);

        let chord = build_chord(Note::A, ChordType::Minor7);
        let expected = vec![Note::A, Note::C, Note::E, Note::G];
        assert_eq!(chord, expected);
    }

    #[test]
    fn test_lib_harmonize_c_major_sevenths() {
        let scale = build_scale(Note::C, ScaleType::Major);
        let harmony = harmonize_scale(&scale, true); // true for 7mas

        let qualities: Vec<ChordType> = harmony.iter().map(|d| d.chord_type).collect();

        let expected_qualities = vec![
            ChordType::Major7,
            ChordType::Minor7,
            ChordType::Minor7,
            ChordType::Major7,
            ChordType::Dominant7,
            ChordType::Minor7,
            ChordType::Minor7b5, // Half-diminished
        ];

        assert_eq!(qualities, expected_qualities);
    }

    #[test]
    fn test_lib_get_inversions() {
        let chord = vec![Note::C, Note::E, Note::G];
        let inversions = get_inversions(&chord);

        let expected = vec![
            vec![Note::C, Note::E, Note::G], // Root
            vec![Note::E, Note::G, Note::C], // 1st
            vec![Note::G, Note::C, Note::E], // 2nd
        ];

        assert_eq!(inversions, expected);

        // Try a 7th chord
        let chord_7 = vec![Note::G, Note::B, Note::D, Note::F];
        let inversions_7 = get_inversions(&chord_7);

        let expected_7 = vec![
            vec![Note::G, Note::B, Note::D, Note::F], // Root
            vec![Note::B, Note::D, Note::F, Note::G], // 1st
            vec![Note::D, Note::F, Note::G, Note::B], // 2nd
            vec![Note::F, Note::G, Note::B, Note::D], // 3rd
        ];

        assert_eq!(inversions_7, expected_7);
    }

    #[test]
    fn test_lib_build_scale_pentatonic() {
        // C Major Pentatonic: C, D, E, G, A
        let scale = build_scale(Note::C, ScaleType::PentatonicMajor);
        let expected = vec![Note::C, Note::D, Note::E, Note::G, Note::A];
        assert_eq!(scale, expected);

        // A Minor Pentatonic: A, C, D, E, G
        let scale = build_scale(Note::A, ScaleType::PentatonicMinor);
        let expected = vec![Note::A, Note::C, Note::D, Note::E, Note::G];
        assert_eq!(scale, expected);
    }

    #[test]
    fn test_lib_build_block_progression_fsharp() {
        let progression = build_progression(Note::FSharp, HarmonicFormula::Block);

        let roots: Vec<Note> = progression.iter().map(|c| c.root_note).collect();
        assert_eq!(
            roots,
            vec![Note::FSharp, Note::CSharp, Note::FSharp, Note::B]
        );

        let types: Vec<ChordType> = progression.iter().map(|c| c.chord_type).collect();
        assert_eq!(
            types,
            vec![
                ChordType::Major,
                ChordType::Dominant7,
                ChordType::Dominant7,
                ChordType::Major
            ]
        );

        // Test F#7 (I7) - Should be root position [F#, A#, C#, E]
        // Previous Bass: F (from V7). Dist to F# is 1. Dist to E is 1.
        // Tie breaker: Centering. F# is Tonic (dist 0). E is dist 2.
        // Winner: Bass F# (Root).
        assert_eq!(
            progression[2].notes,
            vec![Note::FSharp, Note::ASharp, Note::CSharp, Note::E]
        );

        // Test C#7 (V7)
        // Previous Bass: F# (I).
        // Inv Root (C#): Dist F#->C# = 7 or 5? semitone_distance(F#, C#) = 5.
        // Inv 1 (F): Dist F#->F = 1.
        // Inv 2 (G#): Dist F#->G# = 2.
        // Inv 3 (B): Dist F#->B = 5.
        // Winner: Inv 1 (Bass F / E#). Dist 1.
        assert_eq!(
            progression[1].notes,
            vec![Note::F, Note::GSharp, Note::B, Note::CSharp]
        );
    }

    #[test]
    fn test_lib_build_circle_progression_c_major() {
        let progression = build_progression(Note::C, HarmonicFormula::Circle);

        // Check the roots: C, A, D, G
        let roots: Vec<Note> = progression.iter().map(|c| c.root_note).collect();
        assert_eq!(roots, vec![Note::C, Note::A, Note::D, Note::G]);

        // Check the types: Major, Minor, Minor, Dominant7
        let types: Vec<ChordType> = progression.iter().map(|c| c.chord_type).collect();
        assert_eq!(
            types,
            vec![
                ChordType::Major,
                ChordType::Minor,
                ChordType::Minor,
                ChordType::Dominant7
            ]
        );

        // Check the notes of the V7 (G7)
        // Prev Bass: D (from Dm).
        // Inv Root (G): Dist D->G = 5.
        // Inv 1 (B): Dist D->B = 3.
        // Inv 2 (D): Dist D->D = 0.
        // Winner: Inv 2 (Bass D). [D, F, G, B]
        assert_eq!(
            progression[3].notes,
            vec![Note::D, Note::F, Note::G, Note::B]
        );
    }

    #[test]
    fn test_lib_build_guajira_progression_c() {
        let progression = build_progression(Note::C, HarmonicFormula::Guajira);

        // I, IV, V7
        let roots: Vec<Note> = progression.iter().map(|c| c.root_note).collect();
        assert_eq!(roots, vec![Note::C, Note::F, Note::G]);

        let types: Vec<ChordType> = progression.iter().map(|c| c.chord_type).collect();
        assert_eq!(
            types,
            vec![ChordType::Major, ChordType::Major, ChordType::Dominant7]
        );
    }

    #[test]
    fn test_lib_build_bloque_rm_progression_c() {
        let progression = build_progression(Note::C, HarmonicFormula::BloqueRm);

        // vi, IV7, ii, III7
        // C Major -> A, F, D, E
        let roots: Vec<Note> = progression.iter().map(|c| c.root_note).collect();
        assert_eq!(roots, vec![Note::A, Note::F, Note::D, Note::E]);

        let types: Vec<ChordType> = progression.iter().map(|c| c.chord_type).collect();
        assert_eq!(
            types,
            vec![
                ChordType::Minor,
                ChordType::Dominant7,
                ChordType::Minor,
                ChordType::Dominant7
            ]
        );
    }
}
