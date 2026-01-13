/// Entity extraction for ambient contact management
/// High-performance extraction of people mentions, relationships, and facts

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Extracted entity from text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractedEntity {
    pub name: String,
    pub relationship_hint: Option<String>,
    pub relationship_context: String,
    pub pronouns: Option<String>,
    pub mention_context: String,
    pub sentiment: Option<String>,
    pub confidence: f64,
    pub position: usize,
}

/// Relationship pattern definition
#[derive(Debug, Clone)]
struct RelationshipPattern {
    pattern: Regex,
    relationship: &'static str,
    #[allow(dead_code)]
    category: &'static str,
}

/// Entity extraction result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityExtractionResult {
    pub entities: Vec<ExtractedEntity>,
    pub relationship_count: usize,
    pub processing_time_us: u64,
}

/// Words to exclude from name matching
const EXCLUDED_WORDS: &[&str] = &[
    "my", "the", "a", "an", "i", "me", "we", "you", "he", "she", "it", "they",
    "this", "that", "these", "those", "who", "what", "when", "where", "why", "how",
    "today", "yesterday", "tomorrow", "monday", "tuesday", "wednesday", "thursday",
    "friday", "saturday", "sunday", "january", "february", "march", "april", "may",
    "june", "july", "august", "september", "october", "november", "december",
    "just", "really", "very", "also", "too", "even", "still", "already",
    "talked", "said", "told", "asked", "called", "met", "saw", "went",
    "good", "great", "bad", "nice", "happy", "sad", "angry", "upset",
    "dinner", "lunch", "breakfast", "meeting", "conversation", "call", "text",
    "last", "next", "first", "new", "old", "other", "another",
];

lazy_static::lazy_static! {
    /// Pre-compiled relationship patterns for performance
    static ref RELATIONSHIP_PATTERNS: Vec<RelationshipPattern> = vec![
        // Family - possessive patterns
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:mom|mother|mommy|mama)\b").unwrap(), relationship: "mother", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:dad|father|daddy|papa)\b").unwrap(), relationship: "father", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:parents?)\b").unwrap(), relationship: "parent", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:brother|bro)\b").unwrap(), relationship: "brother", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:sister|sis)\b").unwrap(), relationship: "sister", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:sibling)\b").unwrap(), relationship: "sibling", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:son)\b").unwrap(), relationship: "son", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:daughter)\b").unwrap(), relationship: "daughter", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:kid|child)\b").unwrap(), relationship: "child", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:grandma|grandmother|nana|granny)\b").unwrap(), relationship: "grandmother", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:grandpa|grandfather|papa|gramps)\b").unwrap(), relationship: "grandfather", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:aunt|auntie)\b").unwrap(), relationship: "aunt", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:uncle)\b").unwrap(), relationship: "uncle", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:cousin)\b").unwrap(), relationship: "cousin", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:niece)\b").unwrap(), relationship: "niece", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:nephew)\b").unwrap(), relationship: "nephew", category: "family" },

        // Extended family
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:step-?mom|step-?mother|stepmom|stepmother)\b").unwrap(), relationship: "step_mother", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:step-?dad|step-?father|stepdad|stepfather)\b").unwrap(), relationship: "step_father", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:mother-?in-?law|MIL)\b").unwrap(), relationship: "mother_in_law", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:father-?in-?law|FIL)\b").unwrap(), relationship: "father_in_law", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:brother-?in-?law|BIL)\b").unwrap(), relationship: "brother_in_law", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:sister-?in-?law|SIL)\b").unwrap(), relationship: "sister_in_law", category: "family" },

        // Co-parenting
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:co-?parent|coparent)\b").unwrap(), relationship: "co_parent", category: "family" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:ex|ex-?husband|ex-?wife).{0,20}(?:co-?parent|parent|custody)\b").unwrap(), relationship: "ex_spouse_co_parent", category: "family" },

        // Romantic relationships
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:husband|hubby)\b").unwrap(), relationship: "husband", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:wife|wifey)\b").unwrap(), relationship: "wife", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:spouse)\b").unwrap(), relationship: "spouse", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:partner)\b").unwrap(), relationship: "partner", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:SO|significant other)\b").unwrap(), relationship: "significant_other", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:boyfriend|bf)\b").unwrap(), relationship: "boyfriend", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:girlfriend|gf)\b").unwrap(), relationship: "girlfriend", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:fiance|fiancé)\b").unwrap(), relationship: "fiance", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:fiancee|fiancée)\b").unwrap(), relationship: "fiancee", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:ex)\b").unwrap(), relationship: "ex_partner", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:ex-?boyfriend|ex-?girlfriend|ex-?partner)\b").unwrap(), relationship: "ex_partner", category: "romantic" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:ex-?husband|ex-?wife|former spouse)\b").unwrap(), relationship: "ex_spouse", category: "romantic" },

        // Friends
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:best friend|bestie|BFF)\b").unwrap(), relationship: "best_friend", category: "friend" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:close friend)\b").unwrap(), relationship: "close_friend", category: "friend" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:friend)\b").unwrap(), relationship: "friend", category: "friend" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:roommate|flatmate|housemate)\b").unwrap(), relationship: "roommate", category: "friend" },

        // Professional
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:boss|manager|supervisor)\b").unwrap(), relationship: "boss", category: "professional" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:coworker|co-?worker|colleague)\b").unwrap(), relationship: "colleague", category: "professional" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:employee|direct report|team member)\b").unwrap(), relationship: "direct_report", category: "professional" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:mentor)\b").unwrap(), relationship: "mentor", category: "professional" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:mentee)\b").unwrap(), relationship: "mentee", category: "professional" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:client)\b").unwrap(), relationship: "client", category: "professional" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:teacher|professor|instructor)\b").unwrap(), relationship: "teacher", category: "professional" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:student)\b").unwrap(), relationship: "student", category: "professional" },

        // Healthcare/support
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:therapist|counselor|psychologist|psychiatrist)\b").unwrap(), relationship: "therapist", category: "service_provider" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:doctor|physician|GP)\b").unwrap(), relationship: "doctor", category: "service_provider" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:coach)\b").unwrap(), relationship: "coach", category: "service_provider" },

        // Other
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:neighbor|neighbour)\b").unwrap(), relationship: "neighbor", category: "other" },
        RelationshipPattern { pattern: Regex::new(r"(?i)\bmy (?:landlord)\b").unwrap(), relationship: "landlord", category: "other" },
    ];

    /// Pattern to find names after relationship mentions
    static ref NAME_AFTER_RELATION: Regex = Regex::new(r"^\s*,?\s*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)?)\b").unwrap();

    /// Pattern to find any capitalized name
    static ref CAPITALIZED_NAME: Regex = Regex::new(r"\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+)?)\b").unwrap();

    /// Pattern for "Name, my relation" format
    static ref NAME_THEN_RELATION: Regex = Regex::new(r"(?i)\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+)?),?\s+(?:my|who is my|who's my)\s+(\w+(?:-\w+)?)\b").unwrap();

    /// Pronoun patterns
    static ref HE_HIM_PATTERN: Regex = Regex::new(r"(?i)\b(he|him|his|himself)\b").unwrap();
    static ref SHE_HER_PATTERN: Regex = Regex::new(r"(?i)\b(she|her|hers|herself)\b").unwrap();
    static ref THEY_THEM_PATTERN: Regex = Regex::new(r"(?i)\b(they|them|their|theirs|themselves)\b").unwrap();

    /// Sentiment patterns
    static ref POSITIVE_SENTIMENT: Regex = Regex::new(r"(?i)\b(love|happy|grateful|appreciate|enjoy|like|wonderful|great|amazing|fantastic|supportive|helpful|kind|caring)\b").unwrap();
    static ref NEGATIVE_SENTIMENT: Regex = Regex::new(r"(?i)\b(hate|angry|frustrated|annoyed|upset|disappointed|sad|hurt|betrayed|difficult|problematic|toxic|abusive)\b").unwrap();

    /// Excluded words set for fast lookup
    static ref EXCLUDED_SET: HashSet<&'static str> = EXCLUDED_WORDS.iter().cloned().collect();
}

/// Check if a word is a valid name
fn is_valid_name(word: &str) -> bool {
    if word.len() < 2 {
        return false;
    }

    let lower = word.to_lowercase();
    if EXCLUDED_SET.contains(lower.as_str()) {
        return false;
    }

    // Check first character is uppercase
    word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
}

/// Extract name from possessive match like "my mom" -> "mom"
fn extract_name_from_possessive_match(match_text: &str) -> Option<String> {
    let words: Vec<&str> = match_text.split_whitespace().collect();
    if words.len() >= 2 && words[0].to_lowercase() == "my" {
        let name = words[1];
        if name.len() >= 2 && name.chars().all(|c| c.is_alphabetic()) {
            Some(name.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

/// Detect pronouns from context
fn detect_pronouns(context: &str) -> Option<String> {
    let he_count = HE_HIM_PATTERN.find_iter(context).count();
    let she_count = SHE_HER_PATTERN.find_iter(context).count();
    let they_count = THEY_THEM_PATTERN.find_iter(context).count();

    if he_count > 0 && he_count > she_count && he_count > they_count {
        Some("he/him".to_string())
    } else if she_count > 0 && she_count > he_count && she_count > they_count {
        Some("she/her".to_string())
    } else if they_count > 0 {
        Some("they/them".to_string())
    } else {
        None
    }
}

/// Detect sentiment from context
fn detect_sentiment(context: &str) -> Option<String> {
    let positive_count = POSITIVE_SENTIMENT.find_iter(context).count();
    let negative_count = NEGATIVE_SENTIMENT.find_iter(context).count();

    if positive_count > negative_count && positive_count > 0 {
        Some("positive".to_string())
    } else if negative_count > positive_count && negative_count > 0 {
        Some("negative".to_string())
    } else if positive_count > 0 && negative_count > 0 {
        Some("mixed".to_string())
    } else {
        None
    }
}

/// Extract entities from text using pre-compiled regex patterns
pub fn extract_entities(text: &str) -> EntityExtractionResult {
    use std::time::Instant;
    let start = Instant::now();

    let mut entities: Vec<ExtractedEntity> = Vec::with_capacity(10);
    let mut processed_names: HashSet<String> = HashSet::new();

    // Extract from relationship patterns
    for rp in RELATIONSHIP_PATTERNS.iter() {
        if let Some(mat) = rp.pattern.find(text) {
            let match_text = mat.as_str();
            let match_start = mat.start();
            let match_end = mat.end();

            // Get context around the match
            let context_start = match_start.saturating_sub(50);
            let context_end = (match_end + 50).min(text.len());
            let context = &text[context_start..context_end];

            // Look for name after the relationship mention
            let after_match = &text[match_end..];
            let name = if let Some(name_cap) = NAME_AFTER_RELATION.captures(after_match) {
                let potential_name = name_cap.get(1).map(|m| m.as_str()).unwrap_or("");
                if is_valid_name(potential_name) {
                    potential_name.to_string()
                } else {
                    extract_name_from_possessive_match(match_text)
                        .unwrap_or_else(|| find_best_name_in_context(context))
                }
            } else {
                extract_name_from_possessive_match(match_text)
                    .unwrap_or_else(|| find_best_name_in_context(context))
            };

            let name_lower = name.to_lowercase();
            if !processed_names.contains(&name_lower) {
                processed_names.insert(name_lower);

                entities.push(ExtractedEntity {
                    name,
                    relationship_hint: Some(rp.relationship.to_string()),
                    relationship_context: match_text.to_string(),
                    pronouns: detect_pronouns(context),
                    mention_context: context.trim().to_string(),
                    sentiment: detect_sentiment(context),
                    confidence: 0.8,
                    position: match_start,
                });
            }
        }
    }

    // Extract "Name, my relation" pattern
    for cap in NAME_THEN_RELATION.captures_iter(text) {
        if let (Some(name_match), Some(relation_match)) = (cap.get(1), cap.get(2)) {
            let name = name_match.as_str();
            let relation_word = relation_match.as_str().to_lowercase();

            let name_lower = name.to_lowercase();
            if !processed_names.contains(&name_lower) && is_valid_name(name) {
                processed_names.insert(name_lower);

                // Map relation word to relationship type
                let relationship_hint = infer_relationship_from_word(&relation_word);

                let context_start = name_match.start().saturating_sub(30);
                let context_end = (relation_match.end() + 30).min(text.len());
                let context = &text[context_start..context_end];

                entities.push(ExtractedEntity {
                    name: name.to_string(),
                    relationship_hint,
                    relationship_context: cap.get(0).map(|m| m.as_str()).unwrap_or("").to_string(),
                    pronouns: detect_pronouns(context),
                    mention_context: context.trim().to_string(),
                    sentiment: detect_sentiment(context),
                    confidence: 0.85,
                    position: name_match.start(),
                });
            }
        }
    }

    let elapsed = start.elapsed();
    let relationship_count = entities.iter().filter(|e| e.relationship_hint.is_some()).count();

    EntityExtractionResult {
        entities,
        relationship_count,
        processing_time_us: elapsed.as_micros() as u64,
    }
}

/// Find the best name candidate in context
fn find_best_name_in_context(context: &str) -> String {
    for cap in CAPITALIZED_NAME.captures_iter(context) {
        if let Some(m) = cap.get(1) {
            let potential_name = m.as_str();
            if is_valid_name(potential_name) {
                return potential_name.to_string();
            }
        }
    }

    // Fallback: extract relationship term
    context
        .split_whitespace()
        .find(|w| w.starts_with("my"))
        .map(|_| {
            context
                .split_whitespace()
                .skip_while(|w| *w != "my")
                .nth(1)
                .unwrap_or("unknown")
        })
        .unwrap_or("unknown")
        .to_string()
}

/// Infer relationship type from common words
fn infer_relationship_from_word(word: &str) -> Option<String> {
    match word {
        "mom" | "mother" | "mama" | "mommy" => Some("mother".to_string()),
        "dad" | "father" | "papa" | "daddy" => Some("father".to_string()),
        "brother" | "bro" => Some("brother".to_string()),
        "sister" | "sis" => Some("sister".to_string()),
        "husband" | "hubby" => Some("husband".to_string()),
        "wife" | "wifey" => Some("wife".to_string()),
        "spouse" => Some("spouse".to_string()),
        "partner" => Some("partner".to_string()),
        "boyfriend" | "bf" => Some("boyfriend".to_string()),
        "girlfriend" | "gf" => Some("girlfriend".to_string()),
        "friend" => Some("friend".to_string()),
        "boss" | "manager" => Some("boss".to_string()),
        "coworker" | "colleague" => Some("colleague".to_string()),
        "therapist" | "counselor" => Some("therapist".to_string()),
        "doctor" | "physician" => Some("doctor".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_family_relationships() {
        let text = "I talked to my mom about the situation. My dad was also there.";
        let result = extract_entities(text);

        assert!(result.entities.iter().any(|e| e.relationship_hint == Some("mother".to_string())));
        assert!(result.entities.iter().any(|e| e.relationship_hint == Some("father".to_string())));
    }

    #[test]
    fn test_extract_romantic_relationships() {
        let text = "My husband John said we should take a vacation.";
        let result = extract_entities(text);

        let husband_entity = result.entities.iter().find(|e| e.relationship_hint == Some("husband".to_string()));
        assert!(husband_entity.is_some());
        assert_eq!(husband_entity.unwrap().name, "John");
    }

    #[test]
    fn test_extract_named_entities() {
        let text = "Sarah, my sister, called yesterday.";
        let result = extract_entities(text);

        let sarah_entity = result.entities.iter().find(|e| e.name == "Sarah");
        assert!(sarah_entity.is_some());
        assert_eq!(sarah_entity.unwrap().relationship_hint, Some("sister".to_string()));
    }

    #[test]
    fn test_detect_pronouns() {
        let context = "My sister went to the store. She was happy about the sale.";
        let pronouns = detect_pronouns(context);
        assert_eq!(pronouns, Some("she/her".to_string()));
    }

    #[test]
    fn test_detect_sentiment() {
        let positive = "I love spending time with my mom. She's so supportive.";
        assert_eq!(detect_sentiment(positive), Some("positive".to_string()));

        let negative = "I'm frustrated with my boss. He's so difficult.";
        assert_eq!(detect_sentiment(negative), Some("negative".to_string()));
    }
}
