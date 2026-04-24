use crate::indicators::collect_highlights;
use crate::models::{DetectionResult, DetectorConfig, IndicatorBreakdown};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct TextStats {
    pub sentence_count: usize,
    pub burstiness_index: f64,
    pub entropy: f64,
    pub type_token_ratio: f64,
}

pub fn calculate_text_stats(text: &str) -> TextStats {
    if text.is_empty() {
        return TextStats::default();
    }

    let sentences: Vec<&str> = text
        .split(&['.', '!', '?', '\n'][..])
        .map(|sentence| sentence.trim())
        .filter(|sentence| !sentence.is_empty())
        .collect();

    if sentences.is_empty() {
        return TextStats::default();
    }

    let sentence_count = sentences.len();
    let lengths: Vec<usize> = sentences
        .iter()
        .map(|sentence| sentence.split_whitespace().count())
        .collect();

    let average_length = lengths.iter().sum::<usize>() as f64 / sentence_count as f64;

    let variance = if sentence_count > 1 {
        let sum_squared_diff: f64 = lengths
            .iter()
            .map(|&length| (length as f64 - average_length).powi(2))
            .sum();
        sum_squared_diff / (sentence_count - 1) as f64
    } else {
        0.0
    };

    // Coefficient of variation: low = uniform (AI-like), high = varied (human-like)
    let standard_deviation = variance.sqrt();
    let burstiness_index = if average_length > 0.0 {
        standard_deviation / average_length
    } else {
        0.0
    };

    let entropy = calculate_entropy(text);

    let words: Vec<&str> = text.split_whitespace().collect();
    let type_token_ratio = if words.is_empty() {
        0.0
    } else {
        let unique_words: HashSet<&str> = words.iter().copied().collect();
        unique_words.len() as f64 / words.len() as f64
    };

    TextStats {
        sentence_count,
        burstiness_index,
        entropy,
        type_token_ratio,
    }
}

fn calculate_entropy(text: &str) -> f64 {
    if text.is_empty() {
        return 0.0;
    }

    let mut character_counts: HashMap<char, usize> = HashMap::new();
    let total_characters = text
        .chars()
        .filter(|character| character.is_alphabetic())
        .count();

    if total_characters == 0 {
        return 0.0;
    }

    for character in text.chars() {
        if character.is_alphabetic() {
            *character_counts.entry(character).or_insert(0) += 1;
        }
    }

    let mut entropy = 0.0;
    for &count in character_counts.values() {
        let probability = count as f64 / total_characters as f64;
        if probability > 0.0 {
            entropy -= probability * probability.log2();
        }
    }

    entropy
}

pub fn calculate_score_advanced(
    breakdown: &IndicatorBreakdown,
    stats: &TextStats,
    word_count: usize,
    _config: &DetectorConfig,
) -> f64 {
    if word_count == 0 {
        return 0.0;
    }

    let mut score = 0.0;

    if stats.sentence_count >= 3 {
        let burstiness_score = if stats.burstiness_index < 0.3 {
            15.0
        } else if stats.burstiness_index < 0.45 {
            (0.45 - stats.burstiness_index) * 50.0
        } else if stats.burstiness_index > 0.7 {
            -((stats.burstiness_index - 0.7) * 15.0)
        } else {
            0.0
        };
        score += burstiness_score;
    }

    if stats.entropy > 0.0 && stats.entropy < 3.8 {
        score += (3.8 - stats.entropy) * 15.0;
    }

    if word_count > 50 && stats.type_token_ratio < 0.45 {
        score += (0.45 - stats.type_token_ratio) * 25.0;
    }

    score += calculate_pattern_score(breakdown, word_count);

    score.clamp(0.0, 100.0)
}

/// Density-based scoring: normalizes indicator counts per 100 words with per-indicator caps
/// to prevent short texts from inflating and long texts from zeroing out.
fn calculate_pattern_score(breakdown: &IndicatorBreakdown, word_count: usize) -> f64 {
    if word_count == 0 {
        return 0.0;
    }

    let density = |count: usize, weight: f64, cap: f64| -> f64 {
        let raw = count as f64 * weight * (100.0 / word_count as f64);
        raw.min(cap)
    };

    let mut score = 0.0;

    score += density(breakdown.ai_vocabulary, 2.0, 20.0);
    score += density(breakdown.negative_parallelisms, 4.0, 8.0);
    score += density(breakdown.conversational_filler, 3.0, 6.0);
    score += density(breakdown.ai_self_reference, 6.0, 12.0);
    score += density(breakdown.generalizations, 5.0, 8.0);
    score += density(breakdown.tends_to_patterns, 3.0, 5.0);
    score += density(breakdown.bullet_points, 2.0, 5.0);
    score += density(breakdown.contrast_patterns, 2.0, 4.0);
    score += density(breakdown.promotional_phrases, 3.0, 8.0);
    score += density(breakdown.significance_emphasis, 3.5, 10.0);
    score += density(breakdown.vague_attributions, 4.0, 6.0);
    score += density(breakdown.formatting_artifacts, 3.0, 5.0);
    score += density(breakdown.rule_of_three, 2.0, 4.0);
    score += density(breakdown.philosophical_patterns, 6.0, 8.0);
    score += density(breakdown.self_referential, 8.0, 10.0);
    score += density(breakdown.sentence_fragments, 2.0, 3.0);
    score += density(breakdown.didactic_disclaimers, 4.0, 6.0);
    score += density(breakdown.collaborative_comm, 5.0, 10.0);
    score += density(breakdown.superficial_analyses, 4.0, 8.0);
    score += density(breakdown.section_summaries, 5.0, 5.0);
    score += density(breakdown.emoji_usage, 3.0, 5.0);
    score += density(breakdown.copula_avoidance, 3.0, 6.0);
    score += density(breakdown.notability_emphasis, 3.0, 5.0);
    score += density(breakdown.outline_conclusions, 5.0, 6.0);

    if breakdown.curly_quotes {
        score += 3.0;
    }
    if breakdown.knowledge_cutoff_disclaimer {
        score += 25.0;
    }
    if breakdown.high_comma_density {
        score += 5.0;
    }
    if breakdown.chatgpt_artifacts > 0 {
        score += 30.0;
    }
    if breakdown.placeholder_text > 0 {
        score += 20.0;
    }

    score
}

pub fn get_confidence_level(score: f64, threshold: f64) -> String {
    if score >= threshold + 20.0 {
        "High".to_string()
    } else if score >= threshold - 10.0 {
        "Medium".to_string()
    } else {
        "Low".to_string()
    }
}

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn clean_flagged_phrases(phrases: Vec<String>, max_phrases: usize) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();

    for phrase in phrases {
        let normalized = phrase.to_lowercase();
        if !seen.contains(&normalized) && unique.len() < max_phrases {
            seen.insert(normalized);
            unique.push(phrase);
        }
    }

    unique
}

pub fn create_detection_result(
    text: &str,
    breakdown: IndicatorBreakdown,
    flagged_phrases: Vec<String>,
    config: &DetectorConfig,
) -> DetectionResult {
    let word_count = count_words(text);
    let stats = calculate_text_stats(text);

    let score = calculate_score_advanced(&breakdown, &stats, word_count, config);
    let confidence = get_confidence_level(score, config.ai_threshold);
    let likely_ai_generated = score >= config.ai_threshold;

    let mut cleaned_phrases = clean_flagged_phrases(flagged_phrases, 20);

    if stats.burstiness_index < 0.3 {
        cleaned_phrases.push(format!(
            "Very uniform sentence structure (burstiness: {:.2})",
            stats.burstiness_index
        ));
    }

    if stats.entropy < 4.0 {
        cleaned_phrases.push(format!(
            "Low character entropy ({:.2} bits) - high predictability",
            stats.entropy
        ));
    }

    if stats.type_token_ratio < 0.5 {
        cleaned_phrases.push(format!(
            "Low vocabulary diversity ({:.0}% unique words)",
            stats.type_token_ratio * 100.0
        ));
    }

    let highlights = collect_highlights(text);

    DetectionResult {
        score,
        confidence,
        likely_ai_generated,
        breakdown,
        flagged_phrases: cleaned_phrases,
        highlights,
        word_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indicators;

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Hello world"), 2);
        assert_eq!(count_words("This is a test."), 4);
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_confidence_levels() {
        assert_eq!(get_confidence_level(80.0, 60.0), "High");
        assert_eq!(get_confidence_level(60.0, 60.0), "Medium");
        assert_eq!(get_confidence_level(40.0, 60.0), "Low");
    }

    #[test]
    fn test_clean_flagged_phrases() {
        let phrases = vec![
            "test phrase".to_string(),
            "Test phrase".to_string(),
            "another one".to_string(),
        ];
        let cleaned = clean_flagged_phrases(phrases, 10);
        assert_eq!(cleaned.len(), 2);
    }

    #[test]
    fn test_score_stays_in_bounds() {
        let mut breakdown = IndicatorBreakdown::new();
        breakdown.ai_vocabulary = 5;
        breakdown.negative_parallelisms = 2;

        let text = "This is a test text with some words here. This is a test.";
        let stats = calculate_text_stats(text);
        let config = DetectorConfig::default();
        let score = calculate_score_advanced(&breakdown, &stats, 100, &config);

        assert!(score >= 0.0);
        assert!(score <= 100.0);
    }

    #[test]
    fn test_knowledge_cutoff_heavy_penalty() {
        let mut breakdown = IndicatorBreakdown::new();
        breakdown.knowledge_cutoff_disclaimer = true;

        let text = "This is a test text with some words here. This is a test.";
        let stats = calculate_text_stats(text);
        let config = DetectorConfig::default();
        let score = calculate_score_advanced(&breakdown, &stats, 50, &config);

        assert!(score >= 20.0);
    }

    #[test]
    fn test_default_threshold_is_60() {
        let config = DetectorConfig::default();
        assert_eq!(config.ai_threshold, 60.0);
    }

    #[test]
    fn test_obvious_ai_text_scores_high() {
        let ai_text = "The intricate tapestry of Renaissance art showcases the pivotal role \
            that patronage played in fostering creative expression. Additionally, the meticulous \
            craftsmanship of artists underscores the enduring legacy of this transformative period. \
            Not only did the Renaissance revolutionize artistic techniques, but it also bolstered a \
            comprehensive understanding of human anatomy. The interplay between science and art \
            stands as a testament to the holistic approach. Experts argue that the Renaissance \
            represents a crucial turning point, seamlessly blending innovation with tradition. \
            This groundbreaking period continues to garner attention, highlighting its lasting impact.";

        let (breakdown, flagged) = indicators::analyze_indicators(ai_text);
        let config = DetectorConfig::default();
        let result = create_detection_result(ai_text, breakdown, flagged, &config);

        assert!(
            result.score >= 60.0,
            "Obvious AI text scored only {}, expected >= 60",
            result.score
        );
        assert!(result.likely_ai_generated);
    }

    #[test]
    fn test_human_text_scores_low() {
        let human_text = "I went to the store yesterday and bought some eggs. The cashier was \
            rude but whatever. Then I came home and my cat knocked over a plant. Dirt everywhere. \
            I spent like 20 minutes cleaning it up. My roommate said I should just get fake plants \
            but I like real ones even though they're a pain. Had leftover pizza for dinner. It was \
            fine I guess. Watched some TV and went to bed early because I had work in the morning.";

        let (breakdown, flagged) = indicators::analyze_indicators(human_text);
        let config = DetectorConfig::default();
        let result = create_detection_result(human_text, breakdown, flagged, &config);

        assert!(
            result.score < 60.0,
            "Human text scored {}, expected < 60",
            result.score
        );
        assert!(!result.likely_ai_generated);
    }

    #[test]
    fn test_long_text_doesnt_zero_out() {
        let mut breakdown = IndicatorBreakdown::new();
        breakdown.ai_vocabulary = 10;
        breakdown.significance_emphasis = 5;
        breakdown.negative_parallelisms = 3;

        let config = DetectorConfig::default();
        let text = "word ".repeat(500);
        let stats = calculate_text_stats(&text);
        let score = calculate_score_advanced(&breakdown, &stats, 500, &config);

        assert!(
            score > 5.0,
            "Long text with many indicators scored only {}",
            score
        );
    }

    #[test]
    fn test_empty_text() {
        let config = DetectorConfig::default();
        let breakdown = IndicatorBreakdown::new();
        let stats = calculate_text_stats("");
        let score = calculate_score_advanced(&breakdown, &stats, 0, &config);
        assert_eq!(score, 0.0);
    }
}
