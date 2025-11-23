use clap::ValueEnum;
use serde_wasm_bindgen::to_value;
use tonic_music_core::{
    ChordType, HarmonicFormula, ScaleType, build_chord, build_custom_progression,
    build_progression, build_scale, get_inversions, harmonize_scale, parser::parse_note,
    parser::parse_roman_chord,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_scale(root: &str, scale_type: &str) -> Result<JsValue, JsValue> {
    let root_note: tonic_music_core::Note =
        parse_note(root).map_err(|e: String| JsValue::from_str(&e))?;

    let scale: ScaleType = ScaleType::from_str(scale_type, true)
        .map_err(|e: String| JsValue::from_str(&format!("Invalid scale type: {}", e)))?;

    let notes: Vec<tonic_music_core::Note> = build_scale(root_note, scale);

    Ok(to_value(&notes)?)
}

#[wasm_bindgen]
pub fn get_chord(root: &str, chord_type: &str, inversions: bool) -> Result<JsValue, JsValue> {
    let root_note: tonic_music_core::Note =
        parse_note(root).map_err(|e: String| JsValue::from_str(&e))?;

    let chord: ChordType = ChordType::from_str(chord_type, true)
        .map_err(|e: String| JsValue::from_str(&format!("Invalid chord type: {}", e)))?;

    let notes: Vec<tonic_music_core::Note> = build_chord(root_note, chord);

    if inversions {
        let all_inversions: Vec<Vec<tonic_music_core::Note>> = get_inversions(&notes);
        Ok(to_value(&all_inversions)?)
    } else {
        Ok(to_value(&notes)?)
    }
}

#[wasm_bindgen]
pub fn get_harmonization(root: &str, scale_type: &str, sevenths: bool) -> Result<JsValue, JsValue> {
    let root_note: tonic_music_core::Note =
        parse_note(root).map_err(|e: String| JsValue::from_str(&e))?;

    let scale: ScaleType = ScaleType::from_str(scale_type, true)
        .map_err(|e: String| JsValue::from_str(&format!("Invalid scale type: {}", e)))?;

    let scale_notes: Vec<tonic_music_core::Note> = build_scale(root_note, scale);
    let harmony: Vec<tonic_music_core::HarmonizedDegree> = harmonize_scale(&scale_notes, sevenths);

    Ok(to_value(&harmony)?)
}

#[wasm_bindgen]
pub fn get_progression(root: &str, formula: &str) -> Result<JsValue, JsValue> {
    let root_note: tonic_music_core::Note =
        parse_note(root).map_err(|e: String| JsValue::from_str(&e))?;

    let formula_enum: HarmonicFormula = HarmonicFormula::from_str(formula, true)
        .map_err(|e: String| JsValue::from_str(&format!("Invalid formula: {}", e)))?;

    let progression: Vec<tonic_music_core::ProgressionChord> =
        build_progression(root_note, formula_enum);

    Ok(to_value(&progression)?)
}

#[wasm_bindgen]
pub fn get_custom_progression(root: &str, custom_formula: &str) -> Result<JsValue, JsValue> {
    let root_note: tonic_music_core::Note =
        parse_note(root).map_err(|e: String| JsValue::from_str(&e))?;

    let parts: Vec<&str> = custom_formula
        .split(&['-', ' '][..])
        .filter(|s: &&str| !s.is_empty())
        .collect();
    let specs_res: Result<Vec<tonic_music_core::parser::ParsedRomanChord>, String> =
        parts.into_iter().map(parse_roman_chord).collect();

    let specs: Vec<tonic_music_core::parser::ParsedRomanChord> = specs_res
        .map_err(|e: String| JsValue::from_str(&format!("Invalid custom formula: {}", e)))?;

    let progression: Vec<tonic_music_core::ProgressionChord> =
        build_custom_progression(root_note, specs);

    Ok(to_value(&progression)?)
}

#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
