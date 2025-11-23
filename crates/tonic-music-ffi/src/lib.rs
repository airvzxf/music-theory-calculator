use tonic_music_core::parser::parse_roman_chord;
use tonic_music_core::{
    ChordType, HarmonicFormula, Note, ScaleType, build_chord, build_custom_progression,
    build_progression, build_scale, harmonize_scale,
};

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FfiError {
    #[error("{val}")]
    Generic { val: String },
}

uniffi::setup_scaffolding!();

// --- Note ---

#[derive(uniffi::Enum)]
pub enum FfiNote {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl From<Note> for FfiNote {
    fn from(note: Note) -> Self {
        match note {
            Note::C => FfiNote::C,
            Note::CSharp => FfiNote::CSharp,
            Note::D => FfiNote::D,
            Note::DSharp => FfiNote::DSharp,
            Note::E => FfiNote::E,
            Note::F => FfiNote::F,
            Note::FSharp => FfiNote::FSharp,
            Note::G => FfiNote::G,
            Note::GSharp => FfiNote::GSharp,
            Note::A => FfiNote::A,
            Note::ASharp => FfiNote::ASharp,
            Note::B => FfiNote::B,
        }
    }
}

impl From<FfiNote> for Note {
    fn from(note: FfiNote) -> Self {
        match note {
            FfiNote::C => Note::C,
            FfiNote::CSharp => Note::CSharp,
            FfiNote::D => Note::D,
            FfiNote::DSharp => Note::DSharp,
            FfiNote::E => Note::E,
            FfiNote::F => Note::F,
            FfiNote::FSharp => Note::FSharp,
            FfiNote::G => Note::G,
            FfiNote::GSharp => Note::GSharp,
            FfiNote::A => Note::A,
            FfiNote::ASharp => Note::ASharp,
            FfiNote::B => Note::B,
        }
    }
}

// --- ScaleType ---

#[derive(uniffi::Enum)]
pub enum FfiScaleType {
    Major,
    MinorNatural,
    MinorHarmonic,
    PentatonicMajor,
    PentatonicMinor,
}

impl From<FfiScaleType> for ScaleType {
    fn from(scale: FfiScaleType) -> Self {
        match scale {
            FfiScaleType::Major => ScaleType::Major,
            FfiScaleType::MinorNatural => ScaleType::MinorNatural,
            FfiScaleType::MinorHarmonic => ScaleType::MinorHarmonic,
            FfiScaleType::PentatonicMajor => ScaleType::PentatonicMajor,
            FfiScaleType::PentatonicMinor => ScaleType::PentatonicMinor,
        }
    }
}

// --- ChordType ---

#[derive(uniffi::Enum)]
pub enum FfiChordType {
    Major,
    Minor,
    Diminished,
    Augmented,
    Major7,
    Minor7,
    Dominant7,
    Minor7b5,
    Diminished7,
    MinorMajor7,
    AugmentedMajor7,
}

impl From<ChordType> for FfiChordType {
    fn from(ct: ChordType) -> Self {
        match ct {
            ChordType::Major => FfiChordType::Major,
            ChordType::Minor => FfiChordType::Minor,
            ChordType::Diminished => FfiChordType::Diminished,
            ChordType::Augmented => FfiChordType::Augmented,
            ChordType::Major7 => FfiChordType::Major7,
            ChordType::Minor7 => FfiChordType::Minor7,
            ChordType::Dominant7 => FfiChordType::Dominant7,
            ChordType::Minor7b5 => FfiChordType::Minor7b5,
            ChordType::Diminished7 => FfiChordType::Diminished7,
            ChordType::MinorMajor7 => FfiChordType::MinorMajor7,
            ChordType::AugmentedMajor7 => FfiChordType::AugmentedMajor7,
        }
    }
}

impl From<FfiChordType> for ChordType {
    fn from(ct: FfiChordType) -> Self {
        match ct {
            FfiChordType::Major => ChordType::Major,
            FfiChordType::Minor => ChordType::Minor,
            FfiChordType::Diminished => ChordType::Diminished,
            FfiChordType::Augmented => ChordType::Augmented,
            FfiChordType::Major7 => ChordType::Major7,
            FfiChordType::Minor7 => ChordType::Minor7,
            FfiChordType::Dominant7 => ChordType::Dominant7,
            FfiChordType::Minor7b5 => ChordType::Minor7b5,
            FfiChordType::Diminished7 => ChordType::Diminished7,
            FfiChordType::MinorMajor7 => ChordType::MinorMajor7,
            FfiChordType::AugmentedMajor7 => ChordType::AugmentedMajor7,
        }
    }
}

// --- HarmonicFormula ---

#[derive(uniffi::Enum)]
pub enum FfiHarmonicFormula {
    Block,
    Circle,
    Guajira,
    MinorBlock,
}

impl From<FfiHarmonicFormula> for HarmonicFormula {
    fn from(hf: FfiHarmonicFormula) -> Self {
        match hf {
            FfiHarmonicFormula::Block => HarmonicFormula::Block,
            FfiHarmonicFormula::Circle => HarmonicFormula::Circle,
            FfiHarmonicFormula::Guajira => HarmonicFormula::Guajira,
            FfiHarmonicFormula::MinorBlock => HarmonicFormula::MinorBlock,
        }
    }
}

// --- Structs ---

#[derive(uniffi::Record)]
pub struct FfiHarmonizedDegree {
    pub degree: u32,
    pub root_note: FfiNote,
    pub chord_type: FfiChordType,
    pub notes: Vec<FfiNote>,
}

#[derive(uniffi::Record)]
pub struct FfiProgressionChord {
    pub degree: String,
    pub root_note: FfiNote,
    pub chord_type: FfiChordType,
    pub notes: Vec<FfiNote>,
}

// --- Functions ---

#[uniffi::export]
pub fn get_scale_notes(root: FfiNote, scale_type: FfiScaleType) -> Vec<FfiNote> {
    let core_root: Note = root.into();
    let core_scale: ScaleType = scale_type.into();

    let notes: Vec<Note> = build_scale(core_root, core_scale);

    notes.into_iter().map(|n: Note| FfiNote::from(n)).collect()
}

#[uniffi::export]
pub fn get_chord_notes(root: FfiNote, chord_type: FfiChordType) -> Vec<FfiNote> {
    let core_root: Note = root.into();
    let core_chord: ChordType = chord_type.into();

    let notes: Vec<Note> = build_chord(core_root, core_chord);

    notes.into_iter().map(|n: Note| FfiNote::from(n)).collect()
}

#[uniffi::export]
pub fn get_harmonization(
    root: FfiNote,
    scale_type: FfiScaleType,
    sevenths: bool,
) -> Vec<FfiHarmonizedDegree> {
    let core_root: Note = root.into();
    let core_scale: ScaleType = scale_type.into();

    let scale_notes: Vec<Note> = build_scale(core_root, core_scale);
    let harmony: Vec<tonic_music_core::HarmonizedDegree> = harmonize_scale(&scale_notes, sevenths);

    harmony
        .into_iter()
        .map(
            |h: tonic_music_core::HarmonizedDegree| FfiHarmonizedDegree {
                degree: h.degree as u32,
                root_note: h.root_note.into(),
                chord_type: h.chord_type.into(),
                notes: h
                    .notes
                    .into_iter()
                    .map(|n: Note| FfiNote::from(n))
                    .collect(),
            },
        )
        .collect()
}

#[uniffi::export]
pub fn get_progression(root: FfiNote, formula: FfiHarmonicFormula) -> Vec<FfiProgressionChord> {
    let core_root: Note = root.into();
    let core_formula: HarmonicFormula = formula.into();

    let progression: Vec<tonic_music_core::ProgressionChord> =
        build_progression(core_root, core_formula);

    progression
        .into_iter()
        .map(
            |p: tonic_music_core::ProgressionChord| FfiProgressionChord {
                degree: p.degree,
                root_note: p.root_note.into(),
                chord_type: p.chord_type.into(),
                notes: p
                    .notes
                    .into_iter()
                    .map(|n: Note| FfiNote::from(n))
                    .collect(),
            },
        )
        .collect()
}

#[uniffi::export]
pub fn get_custom_progression(
    root: FfiNote,
    formula_str: String,
) -> Result<Vec<FfiProgressionChord>, FfiError> {
    let core_root: Note = root.into();

    // Split by whitespace or dashes
    let parts: Vec<&str> = formula_str
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s: &&str| !s.is_empty())
        .collect();

    let mut parsed_chords: Vec<tonic_music_core::parser::ParsedRomanChord> = Vec::new();

    for part in parts {
        let part: &str = part;
        // Parse each chord string
        let parsed: tonic_music_core::parser::ParsedRomanChord =
            parse_roman_chord(part).map_err(|e: String| FfiError::Generic {
                val: format!("Failed to parse '{}': {}", part, e),
            })?;
        parsed_chords.push(parsed);
    }

    let progression: Vec<tonic_music_core::ProgressionChord> =
        build_custom_progression(core_root, parsed_chords);

    Ok(progression
        .into_iter()
        .map(
            |p: tonic_music_core::ProgressionChord| FfiProgressionChord {
                degree: p.degree,
                root_note: p.root_note.into(),
                chord_type: p.chord_type.into(),
                notes: p
                    .notes
                    .into_iter()
                    .map(|n: Note| FfiNote::from(n))
                    .collect(),
            },
        )
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_generation() {
        let notes: Vec<FfiNote> = get_scale_notes(FfiNote::C, FfiScaleType::Major);
        assert_eq!(notes.len(), 7);
        // C Major: C, D, E, F, G, A, B
        assert!(matches!(notes[0], FfiNote::C));
        assert!(matches!(notes[2], FfiNote::E));
    }

    #[test]
    fn test_chord_generation() {
        let notes: Vec<FfiNote> = get_chord_notes(FfiNote::C, FfiChordType::Major);
        assert_eq!(notes.len(), 3);
        // C Major Triad: C, E, G
        assert!(matches!(notes[0], FfiNote::C));
        assert!(matches!(notes[1], FfiNote::E));
        assert!(matches!(notes[2], FfiNote::G));
    }

    #[test]
    fn test_harmonization() {
        let harmony: Vec<FfiHarmonizedDegree> =
            get_harmonization(FfiNote::C, FfiScaleType::Major, false);
        assert_eq!(harmony.len(), 7);
        // I degree is C Major
        assert!(matches!(harmony[0].root_note, FfiNote::C));
        assert!(matches!(harmony[0].chord_type, FfiChordType::Major));
    }

    #[test]
    fn test_progression_minor_block() {
        let progression: Vec<FfiProgressionChord> =
            get_progression(FfiNote::C, FfiHarmonicFormula::MinorBlock);
        assert_eq!(progression.len(), 4);

        // Expected: vi (Am), VI7 (A7), ii (Dm), III7 (E7)
        // Roots: A, A, D, E
        assert!(matches!(progression[0].root_note, FfiNote::A));
        assert!(matches!(progression[1].root_note, FfiNote::A));
        assert!(matches!(progression[2].root_note, FfiNote::D));
        assert!(matches!(progression[3].root_note, FfiNote::E));

        // Check types for the changed chord (2nd chord)
        assert!(matches!(progression[1].chord_type, FfiChordType::Dominant7));
    }
}
