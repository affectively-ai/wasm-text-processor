/// Pattern matching for high-entropy detection

use regex::Regex;

/// Pattern match structure
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_type: String,
    pub match_text: String,
    pub position: usize,
    pub severity: String,
    pub weight: f64,
}

/// Match patterns in text
/// Optimized with pre-allocated capacity for common use cases
pub fn match_patterns(text: &str) -> Vec<PatternMatch> {
    // Pre-allocate with estimated capacity (most texts have 0-5 matches)
    let mut matches = Vec::with_capacity(5);

    // Character judgment patterns
    let _character_patterns = vec![
        (
            r"\b(you('re|'re| are| r))\s+(\w+\s+)*(so\s+)?(lazy|selfish|stupid|pathetic|worthless)\b",
            "character_judgment",
            "high",
            1.0,
        ),
        (
            r"\b(you('re|'re| are| r))\s+(a|an|the)\s+(liar|loser|failure|disappointment)\b",
            "character_judgment",
            "high",
            1.0,
        ),
    ];

    // Absolute statement patterns (Expanded 5x)
    let absolute_patterns = vec![
        (r"\byou\s+(\w+\s+)?(always|never|constantly|forever|eternally)\s+\w+", "absolute_statement", "high", 0.9),
        (r"\b(undeniably|unquestionably|indisputably|obviously|clearly)\b", "absolute_certainty", "medium", 0.7),
        (r"\b(everyone|nobody|everybody|no\s+one|all\s+of\s+you)\b", "universalizing", "medium", 0.7),
        (r"\b(totally|wholly|fundamentally|inherently|purely|100%|completely)\b", "absolutism", "medium", 0.7),
        (r"\b(impossible|inconceivable|unthinkable|absurd)\b", "dismissive_absolute", "medium", 0.7),
    ];

    // Character judgment patterns (Expanded 5x)
    let character_patterns = vec![
        (
            r"\b(you('re|\'re| are| r))\s+(\w+\s+)*(so\s+)?(lazy|selfish|stupid|pathetic|worthless|arrogant|incompetent|useless|hypocrite|narcissist|psychopath|sociopath|abuser|monster|evil|toxic|poison|parasite|fraud|fake|liar|cheat)\b",
            "character_judgment",
            "high",
            1.0,
        ),
        (r"\b(disgrace|embarrassment|disappointment|failure|loser|clown|fool|idiot|moron|imbecile)\b", "insult", "high", 0.9),
        (r"\b(vile|disgusting|repulsive|revolting|gross|nasty|creepy)\b", "visceral_judgment", "high", 0.9),
        (r"\b(manipulative|controlling|crazy|psycho|insane|unhinged|mental)\b", "sanity_attack", "high", 1.0),
    ];

    // Dehumanization patterns (Red Flag) (Expanded 5x)
    let dehumanization_patterns = vec![
        (
            r"\b(animals|vermin|rats|snakes|cockroaches|infestation|plague|disease|cancer|parasites|swarm|filth|scum|trash|garbage|waste|bacteria|virus|sickness|pests|demons|subhuman|savages|aliens|invaders|tumor|infection|rot|decay|maggots|lice|leeches)\b",
            "dehumanization",
            "high",
            1.0,
        ),
        (r"\b(it|thing|creature|monster|beast|brute|animal)\b", "objectification", "medium", 0.8), // Context dependent, but high entropy
    ];

    // Gaslighting & Reality Distortion (Expanded 5x)
    let gaslighting_patterns = vec![
        (r"you\s+(don't|never|cannot)\s+remember", "gaslighting", "high", 1.0),
        (r"that\s+(never|didn't|obviously\s+didn't)\s+happen", "gaslighting", "high", 1.0),
        (r"you're\s+(crazy|imagining\s+things|overreacting|paranoid|delusional|hysterical|confused|misremembering)", "gaslighting", "high", 1.0),
        (r"it's\s+all\s+(in\s+your\s+head|made\s+up|fiction|fantasy)", "gaslighting", "high", 1.0),
        (r"(can't|cannot)\s+take\s+a\s+joke", "gaslighting_minimization", "high", 0.9),
        (r"you\s+are\s+being\s+(too\s+sensitive|dramatic|emotional|irrational)", "gaslighting_invalidation", "high", 0.9),
        (r"your\s+(truth|reality|perspective)\s+is\s+(wrong|flawed|twisted)", "reality_denial", "high", 1.0),
    ];

    // Double Bind & Emotional Blackmail (Expanded 5x)
    let double_bind_patterns = vec![
        (r"if\s+you\s+(really|actually|truly)\s+(cared|loved|wanted|tried)", "double_bind", "high", 0.9),
        (r"damned\s+if\s+you\s+do", "double_bind", "medium", 0.8),
        (r"after\s+all\s+I('ve| have)\s+(done|sacrificed|given)", "emotional_blackmail", "medium", 0.8),
        (r"(prove|show)\s+me\s+you\s+(care|love)", "testing_trap", "high", 0.8),
        (r"you\s+would\s+know\s+if\s+you", "mind_reading_expectation", "medium", 0.7),
        (r"I\s+guess\s+I'm\s+just\s+a\s+(terrible|bad)\s+(person|partner|friend)", "victim_guilt_trip", "high", 0.8),
    ];

    // Moral Disengagement (Expanded 5x)
    let moral_disengagement_patterns = vec![
        (r"everyone\s+(does|thinks|says|agrees|knows)", "moral_disengagement", "medium", 0.7),
        (r"just\s+(business|how\s+it\s+is|following\s+orders|doing\s+my\s+job)", "moral_disengagement", "medium", 0.7),
        (r"you're\s+too\s+(sensitive|soft|weak)", "minimization", "high", 0.9),
        (r"(had|have)\s+no\s+choice", "abdication_of_responsibility", "medium", 0.7),
        (r"forced\s+(my|our)\s+hand", "abdication_of_responsibility", "medium", 0.7),
        (r"(deserved|asked\s+for)\s+it", "victim_blaming", "high", 1.0),
        (r"(greater\s+good|necessary\s+evil|collateral\s+damage)", "justification", "high", 0.8),
    ];

    // Dark Triad: Retaliation & Aggression (Expanded 5x)
    let dark_triad_patterns = vec![
        (r"\b(destroyed|ruined|payback|revenge|obliterated|punish|crush|annihilate|expose|humiliate|bury)\b", "retaliation", "high", 1.0),
        (r"(threw|throw)\s+it\s+in\s+my\s+face", "weaponized_vulnerability", "high", 0.9),
        (r"taught\s+(them|him|her)\s+a\s+lesson", "retaliation", "high", 0.9),
        (r"(make|made)\s+(them|him|her)\s+pay", "retaliation", "high", 1.0),
        (r"scorched\s+earth", "extreme_aggression", "high", 1.0),
        (r"burn\s+it\s+(all\s+)?down", "destructive_intent", "high", 1.0),
        (r"take\s+(them|him|her|you)\s+down", "targeted_aggression", "high", 0.9),
    ];

    // Dark Triad: Manipulation (Feigned Ignorance) (Expanded 5x)
    let manipulation_patterns = vec![
        (r"(played|playing)\s+(dumb|stupid|innocent|naive)", "feigned_ignorance", "medium", 0.8),
        (r"pretended\s+not\s+to\s+(know|understand|hear|see)", "feigned_ignorance", "medium", 0.8),
        (r"acted\s+(confused|surprised|shocked)", "feigned_ignorance", "medium", 0.8),
        (r"(innocent|honest)\s+mistake", "minimization_tactic", "medium", 0.6),
        (r"never\s+meant\s+to", "intent_denial", "medium", 0.6),
        (r"misunderstood\s+me", "communication_blame", "medium", 0.6),
        (r"didn't\s+realize", "strategic_incompetence", "medium", 0.6),
    ];

    // Klemperer: Militarization & Polarization (Expanded 5x)
    let propaganda_patterns = vec![
        (r"\b(war\s+on|battle|enemy|troops|combat|front\s+lines|battleground|assault|siege|campaign|crusade|army|soldiers|weapons|threat|danger|existential)\b", "militarization", "medium", 0.8),
        (r"(with\s+us\s+or\s+against|either\s+you|good\s+vs\s+evil|pick\s+a\s+side|no\s+middle\s+ground)", "false_polarization", "high", 0.9),
        (r"(just\s+be\s+positive|look\s+on\s+the\s+bright\s+side|good\s+vibes\s+only)", "toxic_positivity", "medium", 0.7),
        (r"\b(real\s+americans|true\s+patriots|traitors|collaborators|sympathizers|fence\s+sitters)\b", "identity_hijacking", "high", 0.9),
        (r"neutrality\s+is\s+(betrayal|complicity)", "forced_allegiance", "high", 0.8),
    ];

    // Negative Coping Behaviors (Expanded 5x)
    let negative_coping_patterns = vec![
        // Reassurance Seeking
        (r"(tell|told)\s+me\s+it('s|\s+is)\s+okay", "reassurance_seeking", "low", 0.5),
        (r"are\s+you\s+(sure|certain|mad|upset)", "reassurance_seeking", "low", 0.4),
        (r"promise\s+me", "reassurance_seeking", "low", 0.5),
        (r"(do|does)\s+(you|he|she|everyone)\s+(hate|dislike)\s+me", "reassurance_seeking", "medium", 0.6),
        (r"am\s+I\s+(annoying|ugly|stupid|bad|wrong)", "reassurance_seeking", "medium", 0.6),
        (r"validate\s+(me|my\s+feelings)", "reassurance_seeking", "low", 0.4),
        
        // Self-Victimization
        (r"(always|constantly)\s+happens\s+to\s+me", "self_victimization", "medium", 0.7),
        (r"why\s+(does\s+this|me)", "self_victimization", "low", 0.6),
        (r"everyone\s+hates\s+me", "self_victimization", "high", 0.8),
        (r"(cursed|jinxed|unlucky|fated)", "external_locus_of_control", "medium", 0.6),
        (r"world\s+is\s+against\s+me", "self_victimization", "high", 0.8),
        (r"damaged\s+goods", "self_devaluation", "high", 0.8),
        (r"no\s+hope\s+for\s+me", "hopelessness", "high", 0.9),
        
        // Catastrophizing
        (r"\b(disaster|catastrophe|ruined|hopeless|pointless|doomed|nightmare|unbearable)\b", "catastrophizing", "medium", 0.7),
        (r"end\s+of\s+the\s+world", "catastrophizing", "high", 0.8),
        (r"never\s+going\s+to\s+work", "catastrophizing", "medium", 0.7),
        (r"all\s+is\s+lost", "catastrophizing", "high", 0.9),
        (r"game\s+over", "termination_thinking", "medium", 0.6),
        (r"no\s+future", "future_loss", "high", 0.9),
        
        // Displacement (Lashing Out)
        (r"it('s|\s+is)\s+(all\s+)?your\s+fault", "displacement", "high", 0.9),
        (r"you\s+(made|forced|provoked)\s+me", "displacement", "high", 0.9),
        (r"because\s+of\s+you", "displacement", "medium", 0.7),
        (r"look\s+what\s+you\s+(did|caused)", "blame_shifting", "high", 0.8),
        (r"you\s+started\s+it", "childish_blame", "medium", 0.6),
        (r"pushed\s+my\s+buttons", "responsibility_avoidance", "medium", 0.7),
        
        // Withdrawal / Stonewalling
        (r"leave\s+me\s+alone", "withdrawal", "medium", 0.6),
        (r"don't\s+want\s+to\s+(talk|discuss|hear\s+it)", "withdrawal", "medium", 0.6),
        (r"shut\s+(up|it)", "withdrawal", "high", 0.8),
        (r"(going|gone)\s+dark", "withdrawal", "low", 0.5),
        (r"blocking\s+you", "digital_withdrawal", "high", 0.8),
        (r"(ghosting|ghosted)", "withdrawal", "medium", 0.7),
        (r"silent\s+treatment", "punitive_silence", "high", 0.8),
        (r"walling\s+(off|up)", "emotional_barrier", "medium", 0.6),
        
        // Substance / Escapism
        (r"need\s+a\s+(drink|hit|smoke|pill|fix)", "substance_use", "medium", 0.7),
        (r"get\s+(high|drunk|wasted|smashed|hammered|stoned)", "substance_use", "medium", 0.7),
        (r"\b(numb|forget|escape|checked\s+out)\b", "escapism", "low", 0.5),
    ];

    // Clinical / Defense Mechanisms
    let clinical_defense_patterns = vec![
        (r"making\s+me\s+feel\s+(what|how)\s+you\s+feel", "projective_identification", "high", 0.9),
        (r"dumping\s+your\s+(feelings|emotions)\s+on\s+me", "projective_identification", "medium", 0.7),
        (r"(hot\s+and\s+cold|mixed\s+signals|breadcrumbs|push\s+pull)", "intermittent_reinforcement", "high", 0.9),
        (r"(best\s+person|worst\s+enemy)\s+ever", "splitting", "high", 0.9),
        (r"saint\s+or\s+(devil|sinner)", "splitting", "medium", 0.8),
        (r"(perfect|flawless)\s+to\s+(garbage|worthless)", "splitting", "high", 1.0),
    ];

    // High-Control / Coercive Control
    let high_control_patterns = vec![
        (r"(forget|forgotten|lost)\s+who\s+I\s+am", "perspecticide", "high", 1.0),
        (r"my\s+ideas\s+aren't\s+mine", "perspecticide", "high", 1.0),
        (r"brainwashed", "perspecticide", "high", 0.9),
        (r"(monitoring|tracking)\s+my\s+(location|phone|messages)", "coercive_control", "high", 1.0),
        (r"asking\s+permission\s+to", "coercive_control", "high", 0.9),
        (r"(allowance|access)\s+to\s+money", "financial_abuse", "high", 1.0),
        (r"(isolate|cut\s+off)\s+from\s+(friends|family)", "isolation", "high", 1.0),
        (r"he\s+said\s+that\s+you", "triangulation", "medium", 0.7),
        (r"everyone\s+agrees\s+with\s+me", "triangulation", "medium", 0.7),
        (r"pitting\s+us\s+against", "triangulation", "high", 0.9),
    ];

    // Bad Faith / Intellectual / Moral
    let bad_faith_patterns = vec![
        // Sealioning
        (r"(just|merely)\s+asking\s+(questions|a\s+question)", "sealioning", "medium", 0.7),
        (r"debate\s+me", "bad_faith_debate", "high", 0.8),
        (r"define\s+(your\s+terms|racism|sexism|hate)", "sealioning_definitions", "medium", 0.7),
        (r"(citation|source)\s+needed", "bad_faith_pedantry", "low", 0.5),
        
        // Weaponized Intellectualization
        (r"facts\s+(don't|do\s+not)\s+care\s+about\s+your\s+feelings", "weaponized_intellectualization", "high", 0.9),
        (r"(technically|logically)\s+correct", "bad_faith_pedantry", "low", 0.5),
        (r"you('re|r)\s+being\s+(irrational|emotional|illogical)", "weaponized_intellectualization", "medium", 0.8),
        
        // Concern Trolling
        (r"(just|only)\s+worried\s+about\s+you", "concern_trolling", "medium", 0.7),
        (r"for\s+your\s+own\s+good", "concern_trolling", "medium", 0.7),
        (r"hate\s+to\s+see\s+you\s+like\s+this", "concern_trolling", "low", 0.6),
        
        // Moral Grandstanding & Dog Whistling
        (r"I\s+would\s+never", "moral_grandstanding", "medium", 0.6),
        (r"(right|wrong)\s+side\s+of\s+history", "moral_grandstanding", "medium", 0.7),
        (r"(you\s+people|globalists|thugs|urban\s+youth)", "dog_whistling", "medium", 0.8), // Context dependent
        
        // Negging
        (r"(actually|pretty|smart)\s+for\s+a", "negging", "high", 0.9),
        (r"no\s+offense\s+but", "negging", "medium", 0.7),
        (r"don't\s+take\s+this\s+the\s+wrong\s+way", "negging", "medium", 0.6),
        
        // Whataboutism & Tone Policing
        (r"what\s+about", "whataboutism", "medium", 0.7),
        (r"double\s+standard", "whataboutism", "medium", 0.6),
        (r"calm\s+down", "tone_policing", "high", 0.8),
    ];

    // Combine all patterns
    let all_patterns: Vec<(&str, &str, &str, f64)> = character_patterns
        .into_iter()
        .chain(absolute_patterns.into_iter())
        .chain(dehumanization_patterns.into_iter())
        .chain(gaslighting_patterns.into_iter())
        .chain(double_bind_patterns.into_iter())
        .chain(moral_disengagement_patterns.into_iter())
        .chain(dark_triad_patterns.into_iter())
        .chain(manipulation_patterns.into_iter())
        .chain(propaganda_patterns.into_iter())
        .chain(negative_coping_patterns.into_iter())
        .chain(clinical_defense_patterns.into_iter())
        .chain(high_control_patterns.into_iter())
        .chain(bad_faith_patterns.into_iter())
        .collect();

    for (pattern_str, pattern_type, severity, weight) in all_patterns {
        // Make regex case-insensitive
        let case_insensitive_pattern = format!("(?i){}", pattern_str);
        if let Ok(regex) = Regex::new(&case_insensitive_pattern) {
            for cap in regex.find_iter(text) {
                matches.push(PatternMatch {
                    pattern_type: pattern_type.to_string(),
                    match_text: cap.as_str().to_string(),
                    position: cap.start(),
                    severity: severity.to_string(),
                    weight,
                });
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_patterns() {
        let text = "You are always so lazy";
        let matches = match_patterns(text);
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_character_judgment() {
        let text = "You're so selfish";
        let matches = match_patterns(text);
        assert!(matches.iter().any(|m| m.pattern_type == "character_judgment"));
    }
}
