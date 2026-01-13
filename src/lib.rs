use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

mod pattern_matching;
mod scoring;
mod entity_extraction;

use pattern_matching::match_patterns;
use scoring::calculate_text_score;
use entity_extraction::extract_entities;

/// Pattern match result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternMatchResult {
    pub pattern_type: String,
    pub match_text: String,
    pub position: usize,
    pub severity: String,
    pub weight: f64,
}

/// Text processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextProcessingResult {
    pub detected: bool,
    pub confidence: f64,
    pub patterns: Vec<PatternMatchResult>,
    pub score: f64,
}

/// Detect high-entropy patterns in text
/// 
/// # Arguments
/// * `text` - Text to analyze
/// 
/// # Returns
/// JSON string with detection results
#[wasm_bindgen]
pub fn detect_high_entropy_patterns(text: &str) -> String {
    let matches = match_patterns(text);
    let score = calculate_text_score(&matches);
    let detected = score > 0.3; // Threshold for detection
    let confidence = score.min(1.0);

    let pattern_results: Vec<PatternMatchResult> = matches
        .iter()
        .map(|m| PatternMatchResult {
            pattern_type: m.pattern_type.clone(),
            match_text: m.match_text.clone(),
            position: m.position,
            severity: m.severity.clone(),
            weight: m.weight,
        })
        .collect();

    let result = TextProcessingResult {
        detected,
        confidence,
        patterns: pattern_results,
        score,
    };

    match serde_json::to_string(&result) {
        Ok(json) => json,
        Err(_) => r#"{"detected":false,"confidence":0.0,"patterns":[],"score":0.0}"#.to_string(),
    }
}

/// Extract keywords from text
/// 
/// # Arguments
/// * `text` - Text to analyze
/// 
/// # Returns
/// JSON array of keywords
#[wasm_bindgen]
pub fn extract_keywords(text: &str) -> String {
    use regex::Regex;
    
    // Simple keyword extraction - look for important words
    let keyword_patterns = vec![
        r"\b(you|your|always|never|constantly|selfish|lazy|stupid|idiot|hate|blame|fault)\b",
        r"\b(terrible|awful|horrible|worthless|useless|pathetic|incompetent)\b",
        r"\b(manipulative|narcissist|abuser|psycho|sociopath|liar|loser)\b",
    ];

    let mut keywords: Vec<String> = Vec::new();
    
    for pattern_str in keyword_patterns {
        if let Ok(regex) = Regex::new(pattern_str) {
            for cap in regex.find_iter(text) {
                keywords.push(cap.as_str().to_lowercase());
            }
        }
    }

    // Remove duplicates
    keywords.sort();
    keywords.dedup();

    match serde_json::to_string(&keywords) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    }
}

/// Extract people entities from text (for ambient contact management)
/// 
/// # Arguments
/// * `text` - Text to analyze for people mentions
/// 
/// # Returns
/// JSON string with extracted entities including names, relationships, and context
#[wasm_bindgen]
pub fn extract_people_entities(text: &str) -> String {
    let result = extract_entities(text);
    
    match serde_json::to_string(&result) {
        Ok(json) => json,
        Err(_) => r#"{"entities":[],"relationshipCount":0,"processingTimeUs":0}"#.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_high_entropy_patterns() {
        let text = "You are always so lazy and selfish";
        let result = detect_high_entropy_patterns(text);
        assert!(result.contains("detected"));
    }

    #[test]
    fn test_detect_dehumanization() {
        let text = "They are just a plague of vermin";
        let result = detect_high_entropy_patterns(text);
        assert!(result.contains("dehumanization"));
        assert!(result.contains("vermin"));
    }

    #[test]
    fn test_detect_gaslighting() {
        let text = "You know that never happened, you're crazy";
        let result = detect_high_entropy_patterns(text);
        assert!(result.contains("gaslighting"));
    }

    #[test]
    fn test_detect_double_bind() {
        let text = "If you really cared about me, you would do this";
        let result = detect_high_entropy_patterns(text);
        assert!(result.contains("double_bind"));
    }

    #[test]
    fn test_detect_dark_triad() {
        let text = "I will get my revenge and they will be ruined";
        let result = detect_high_entropy_patterns(text);
        assert!(result.contains("retaliation"));
    }

    #[test]
    fn test_detect_propaganda() {
        let text = "He is an enemy of the people, you are either with us or against us";
        let result = detect_high_entropy_patterns(text);
        assert!(result.contains("militarization"));
        assert!(result.contains("false_polarization"));
    }

    #[test]
    fn test_detect_negative_coping() {
        // Reassurance Seeking
        let text_reassurance = "Tell me it's okay, promise me";
        let result_reassurance = detect_high_entropy_patterns(text_reassurance);
        assert!(result_reassurance.contains("reassurance_seeking"));

        // Self-Victimization
        let text_victim = "Why does this always happen to me?";
        let result_victim = detect_high_entropy_patterns(text_victim);
        assert!(result_victim.contains("self_victimization"));

        // Displacement
        let text_displacement = "It is all your fault that I am like this";
        let result_displacement = detect_high_entropy_patterns(text_displacement);
        assert!(result_displacement.contains("displacement"));

        // Withdrawal
        let text_withdrawal = "Leave me alone, I don't want to talk";
        let result_withdrawal = detect_high_entropy_patterns(text_withdrawal);
        assert!(result_withdrawal.contains("withdrawal"));
    }

    #[test]
    fn test_detect_advanced_patterns() {
        // Clinical / Defense
        let text_proj = "Stop making me feel what you feel";
        let result_proj = detect_high_entropy_patterns(text_proj);
        assert!(result_proj.contains("projective_identification"));
        
        let text_splitting = "You are the best person ever, actually you are garbage";
        let result_splitting = detect_high_entropy_patterns(text_splitting);
        assert!(result_splitting.contains("splitting"));

        // High Control
        let text_perspecticide = "I have forgotten who I am because of you";
        let result_perspecticide = detect_high_entropy_patterns(text_perspecticide);
        assert!(result_perspecticide.contains("perspecticide"));
        
        let text_coercive = "He is always monitoring my location";
        let result_coercive = detect_high_entropy_patterns(text_coercive);
        assert!(result_coercive.contains("coercive_control"));

        // Bad Faith / Intellectual
        let text_sealion = "I am just asking questions about your data";
        let result_sealion = detect_high_entropy_patterns(text_sealion);
        assert!(result_sealion.contains("sealioning"));

        let text_negging = "You are actually pretty for a smart girl";
        let result_negging = detect_high_entropy_patterns(text_negging);
        assert!(result_negging.contains("negging"));
        
        let text_intel = "Facts don't care about your feelings, you're being irrational";
        let result_intel = detect_high_entropy_patterns(text_intel);
        assert!(result_intel.contains("weaponized_intellectualization"));
    }

    #[test]
    fn test_extract_keywords() {
        let text = "You are always so lazy";
        let result = extract_keywords(text);
        assert!(result.contains("you") || result.contains("always") || result.contains("lazy"));
    }
}
