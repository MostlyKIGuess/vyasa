use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetectRequest {
    pub text: String,
    #[serde(default)]
    pub max_chars: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndicatorBreakdown {
    pub ai_vocabulary: usize,
    pub negative_parallelisms: usize,
    pub promotional_phrases: usize,
    pub significance_emphasis: usize,
    pub vague_attributions: usize,
    pub formatting_artifacts: usize,
    pub rule_of_three: usize,
    pub curly_quotes: bool,
    pub knowledge_cutoff_disclaimer: bool,
    pub conversational_filler: usize,
    pub ai_self_reference: usize,
    pub tends_to_patterns: usize,
    pub bullet_points: usize,
    pub contrast_patterns: usize,
    pub generalizations: usize,
    pub philosophical_patterns: usize,
    pub self_referential: usize,
    pub sentence_fragments: usize,
    pub high_comma_density: bool,
    pub didactic_disclaimers: usize,
    pub collaborative_comm: usize,
    pub superficial_analyses: usize,
    pub section_summaries: usize,
    pub emoji_usage: usize,
    pub placeholder_text: usize,
    pub chatgpt_artifacts: usize,
    pub copula_avoidance: usize,
    pub notability_emphasis: usize,
    pub outline_conclusions: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Highlight {
    pub start: usize,
    pub end: usize,
    pub category: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DetectionResult {
    pub score: f64,
    pub confidence: String,
    pub likely_ai_generated: bool,
    pub breakdown: IndicatorBreakdown,
    pub flagged_phrases: Vec<String>,
    pub highlights: Vec<Highlight>,
    pub word_count: usize,
}

impl IndicatorBreakdown {
    pub fn new() -> Self {
        Self {
            ai_vocabulary: 0,
            negative_parallelisms: 0,
            promotional_phrases: 0,
            significance_emphasis: 0,
            vague_attributions: 0,
            formatting_artifacts: 0,
            rule_of_three: 0,
            curly_quotes: false,
            knowledge_cutoff_disclaimer: false,
            conversational_filler: 0,
            ai_self_reference: 0,
            tends_to_patterns: 0,
            bullet_points: 0,
            contrast_patterns: 0,
            generalizations: 0,
            philosophical_patterns: 0,
            self_referential: 0,
            sentence_fragments: 0,
            high_comma_density: false,
            didactic_disclaimers: 0,
            collaborative_comm: 0,
            superficial_analyses: 0,
            section_summaries: 0,
            emoji_usage: 0,
            placeholder_text: 0,
            chatgpt_artifacts: 0,
            copula_avoidance: 0,
            notability_emphasis: 0,
            outline_conclusions: 0,
        }
    }
}

impl Default for IndicatorBreakdown {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct DetectorConfig {
    pub ai_threshold: f64,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self { ai_threshold: 60.0 }
    }
}
