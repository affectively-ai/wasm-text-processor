/// Text scoring algorithms

use super::pattern_matching::PatternMatch;

/// Calculate overall text score from pattern matches
pub fn calculate_text_score(matches: &[PatternMatch]) -> f64 {
    if matches.is_empty() {
        return 0.0;
    }

    // Sum of weighted matches
    let total_weight: f64 = matches.iter().map(|m| m.weight).sum();
    
    // Normalize by number of matches (more matches = higher confidence)
    let match_count = matches.len() as f64;
    let normalized_score = total_weight / (1.0 + match_count * 0.1);

    // Cap at 1.0
    normalized_score.min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::pattern_matching::PatternMatch;

    #[test]
    fn test_calculate_text_score() {
        let matches = vec![
            PatternMatch {
                pattern_type: "character_judgment".to_string(),
                match_text: "You're lazy".to_string(),
                position: 0,
                severity: "high".to_string(),
                weight: 1.0,
            },
        ];
        let score = calculate_text_score(&matches);
        assert!(score > 0.0);
        assert!(score <= 1.0);
    }

    #[test]
    fn test_calculate_empty_score() {
        let matches: Vec<PatternMatch> = vec![];
        let score = calculate_text_score(&matches);
        assert_eq!(score, 0.0);
    }
}
