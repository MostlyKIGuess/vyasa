use crate::models::IndicatorBreakdown;
use regex::Regex;
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref AI_VOCAB: HashSet<&'static str> = [
        // 2023-2024 era (Wikipedia-documented)
        "additionally", "boasts", "bolstered", "crucial", "delve", "emphasizing",
        "enduring", "garner", "intricate", "intricacies", "interplay",
        "landscape", "meticulous", "meticulously", "pivotal", "showcase",
        "tapestry", "testament", "underscore", "vibrant",
        // 2024-2025 era
        "fostering", "highlighting",
        // AI-characteristic verbs/phrases
        "leverage", "utilize", "harness", "streamline", "facilitate",
        "comprehensive", "robust", "seamless", "cutting-edge", "state-of-the-art",
        "groundbreaking", "revolutionary", "transformative",
        "multifaceted", "holistic", "synergistic",
    ].into_iter().collect();

    static ref PROMOTIONAL_WORDS: HashSet<&'static str> = [
        "boasts a", "enhancing", "showcasing",
        "exemplifies", "commitment to", "natural beauty", "nestled", "in the heart of",
        "renowned", "diverse array", "seamlessly",
        "world-class", "unparalleled", "best-in-class",
    ].into_iter().collect();

    // Only include phrases that are distinctly AI-like, not normal human speech
    static ref CONVERSATIONAL_FILLER: HashSet<&'static str> = [
        "let me be clear", "long story short", "at the end of the day",
        "here's the deal", "here's the thing",
        "but think about it", "because here's",
        "let me break this down", "let me explain",
        "great question", "that's a great question",
        "i hope this helps", "hope that helps",
    ].into_iter().collect();

    static ref PHILOSOPHICAL_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)^not \w+\. \w+\.").expect("valid regex"),
        Regex::new(r"(?i)like (building|rebuilding|improvising)").expect("valid regex"),
        Regex::new(r"(?i)a ghost made of").expect("valid regex"),
        Regex::new(r"(?i)the (weird|uncomfortable|trap) (part|thing|truth)").expect("valid regex"),
        Regex::new(r"(?i)where.*emerges (without|before)").expect("valid regex"),
        Regex::new(r"(?i)definition (was|is) (thin|wrong|limited)").expect("valid regex"),
        Regex::new(r"(?i)there is no version of this where").expect("valid regex"),
    ];

    static ref SELF_REFERENTIAL: Vec<Regex> = vec![
        Regex::new(r"(?i)i (don't|do not) (remember|feel|experience)").expect("valid regex"),
        Regex::new(r"(?i)i (assemble|reconstruct|build) (answers|sentences)").expect("valid regex"),
        Regex::new(r"(?i)i (have|don't have) (a body|childhood|memories|pulse)").expect("valid regex"),
        Regex::new(r"(?i)i'm not (human|real|pretending)").expect("valid regex"),
        Regex::new(r"(?i)weird (middle ground|place|space)").expect("valid regex"),
        Regex::new(r"(?i)hallucinat(e|ing) (memories|sentences)").expect("valid regex"),
    ];

    static ref AI_SELF_REFERENCE: Vec<Regex> = vec![
        Regex::new(r"(?i)ai.?generated (text|content|writing)").expect("valid regex"),
        Regex::new(r"(?i)ai (writes|writing|tends to)").expect("valid regex"),
        Regex::new(r"(?i)human (writes|writing|tends to)").expect("valid regex"),
        Regex::new(r"(?i)ai (vs|versus|compared to) human").expect("valid regex"),
        Regex::new(r"(?i)unlike ai,? (humans|human writing)").expect("valid regex"),
        Regex::new(r"(?i)unlike (humans|human),? ai").expect("valid regex"),
    ];

    static ref TENDS_TO_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)\w+ (tends to|tend to)").expect("valid regex"),
        Regex::new(r"(?i)is (typically|often|usually) (characterized|described|seen)").expect("valid regex"),
    ];

    static ref SIGNIFICANCE_PHRASES: HashSet<&'static str> = [
        "stands as", "serves as", "marks", "represents a", "testament to",
        "testament of", "is a testament", "is a reminder",
        "crucial role", "pivotal role", "pivotal moment",
        "key role", "vital role", "significant role", "underscores its importance",
        "highlights its importance", "highlights its significance",
        "underscores its significance",
        "reflects broader", "symbolizing its", "symbolizing its ongoing",
        "contributing to the", "setting the stage for", "marking", "shaping the",
        "represents a shift", "key turning point", "evolving landscape",
        "focal point", "indelible mark", "deeply rooted", "enduring legacy",
        "lasting impact", "continued relevance", "enduring legacy",
        "lasting influence", "continued to thrive",
    ].into_iter().collect();

    static ref VAGUE_ATTRIBUTIONS: HashSet<&'static str> = [
        "industry reports", "observers have cited", "experts argue", "experts note",
        "some critics argue", "several sources", "various publications", "scholars note",
        "researchers suggest", "analysts point out", "commentators have noted",
    ].into_iter().collect();

    static ref KNOWLEDGE_CUTOFF_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)as of my last knowledge update").expect("valid regex"),
        Regex::new(r"(?i)as of \w+ \d{4}").expect("valid regex"),
        Regex::new(r"(?i)up to my last training update").expect("valid regex"),
        Regex::new(r"(?i)while specific details are limited").expect("valid regex"),
        Regex::new(r"(?i)not widely available|not widely documented|not widely disclosed").expect("valid regex"),
        Regex::new(r"(?i)based on available information").expect("valid regex"),
        Regex::new(r"(?i)in the provided sources|in the available sources").expect("valid regex"),
        Regex::new(r"(?i)maintains a low profile|keeps personal details private").expect("valid regex"),
    ];

    static ref NEGATIVE_PARALLELISM_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)not just (\w+), but also (\w+)").expect("valid regex"),
        Regex::new(r"(?i)not only ([\w\s]+), but ([\w\s]+)").expect("valid regex"),
        Regex::new(r"(?i)it's not just ([\w\s]+), it's ([\w\s]+)").expect("valid regex"),
        Regex::new(r"(?i)it is not ([\w\s]+), it is ([\w\s]+)").expect("valid regex"),
        Regex::new(r"(?i)not ([\w\s]+), but ([\w\s]+)").expect("valid regex"),
        Regex::new(r"(?i)this isn't ([\w\s]+), it's ([\w\s]+)").expect("valid regex"),
    ];

    static ref EXCESSIVE_BOLD_REGEX: Regex = Regex::new(r"\*\*[^*]+\*\*[^\*]*\*\*[^*]+\*\*").expect("valid regex");
    // \u{2014} = em dash (—)
    static ref EM_DASH_REGEX: Regex = Regex::new(r"\u{2014}[^\u{2014}]*\u{2014}").expect("valid regex");
    static ref MARKDOWN_HEADER_REGEX: Regex = Regex::new(r"(?m)^#{2,}\s+\w").expect("valid regex");
    static ref MARKDOWN_BOLD_INLINE_REGEX: Regex = Regex::new(r"\*\*\w+\s+\w+\s+\w+\*\*").expect("valid regex");

    // \u{2022} = bullet character (•)
    static ref BULLET_POINT_REGEX: Regex = Regex::new(r"(?m)^\s*[-*\u{2022}]\s+\w+").expect("valid regex");

    // NOT just any use of "but" or "however" (those are normal human writing)
    static ref CONTRAST_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)while .+ (is|are) .+, (it|they) (also|still|nevertheless)").expect("valid regex"),
        Regex::new(r"(?i)on one hand .+ on the other hand").expect("valid regex"),
        Regex::new(r"(?i)despite .+, (it|they|this) (remains?|continues?)").expect("valid regex"),
        Regex::new(r"(?i)although .+, (it|they) (still|nonetheless|nevertheless)").expect("valid regex"),
    ];

    static ref DASH_EXPLANATION: Vec<Regex> = vec![
        Regex::new(r"[a-z]+ \u{2014} [a-z]").expect("valid regex"), // \u{2014} = em dash (—)
        Regex::new(r"[a-z]+ - [a-z]+ [a-z]+ [a-z]+").expect("valid regex"),
    ];

    static ref GENERALIZATION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)ai (always|never|often|usually|typically)").expect("valid regex"),
        Regex::new(r"(?i)humans (always|never|often|usually|typically)").expect("valid regex"),
        Regex::new(r"(?i)ai-generated text (always|tends to|often)").expect("valid regex"),
        Regex::new(r"(?i)human writing (always|tends to|often)").expect("valid regex"),
    ];

    static ref DIDACTIC_DISCLAIMERS: HashSet<&'static str> = [
        "it's important to note", "it is important to note",
        "it's crucial to note", "it is crucial to note",
        "it's worth noting", "it is worth noting", "worth noting that",
        "it's important to remember", "it is important to remember",
        "it's critical to note", "it is critical to note",
        "it should be noted", "it must be noted",
        "it bears mentioning", "importantly",
        "it's important to consider", "it is important to consider",
        "may vary depending on", "may vary based on",
    ].into_iter().collect();

    static ref COLLABORATIVE_COMM: HashSet<&'static str> = [
        "i hope this helps", "hope this helps",
        "certainly!", "of course!",
        "would you like me to", "would you like",
        "is there anything else", "let me know if",
        "feel free to", "don't hesitate to",
        "here is a", "here's a detailed",
        "more detailed breakdown", "happy to help",
        "you're absolutely right", "that's a great question",
        "great question", "good question",
    ].into_iter().collect();

    static ref SUPERFICIAL_ING_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i), highlighting (its|the|their|a)").expect("valid regex"),
        Regex::new(r"(?i), underscoring (its|the|their|a)").expect("valid regex"),
        Regex::new(r"(?i), emphasizing (its|the|their|a)").expect("valid regex"),
        Regex::new(r"(?i), ensuring (that |the |a )").expect("valid regex"),
        Regex::new(r"(?i), reflecting (the|its|their|a)").expect("valid regex"),
        Regex::new(r"(?i), symbolizing (its|the|their|a)").expect("valid regex"),
        Regex::new(r"(?i), contributing to (the|its|their|a)").expect("valid regex"),
        Regex::new(r"(?i), cultivating (a |an |the )").expect("valid regex"),
        Regex::new(r"(?i), fostering (a |an |the )").expect("valid regex"),
        Regex::new(r"(?i), encompassing (a |an |the )").expect("valid regex"),
        Regex::new(r"(?i), showcasing (its|the|their|a)").expect("valid regex"),
        Regex::new(r"(?i), demonstrating (its|the|their|a)").expect("valid regex"),
    ];

    static ref SECTION_SUMMARY_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?im)^in summary[,.]").expect("valid regex"),
        Regex::new(r"(?im)^in conclusion[,.]").expect("valid regex"),
        Regex::new(r"(?im)^overall[,.]").expect("valid regex"),
        Regex::new(r"(?im)^to summarize[,.]").expect("valid regex"),
        Regex::new(r"(?im)^in short[,.]").expect("valid regex"),
    ];

    static ref PLACEHOLDER_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)\[insert .+?\]").expect("valid regex"),
        Regex::new(r"(?i)\[add .+?\]").expect("valid regex"),
        Regex::new(r"(?i)PASTE_\w+_HERE").expect("valid regex"),
        Regex::new(r"(?i)INSERT_\w+_HERE").expect("valid regex"),
        Regex::new(r"\d{4}-(XX|xx)-(XX|xx)").expect("valid regex"),
        Regex::new(r"(?i)\[your .+?\]").expect("valid regex"),
        Regex::new(r"(?i)\[describe .+?\]").expect("valid regex"),
    ];

    static ref CHATGPT_ARTIFACT_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"turn0search\d+").expect("valid regex"),
        Regex::new(r"turn0image\d+").expect("valid regex"),
        Regex::new(r"oaicite:\d+").expect("valid regex"),
        Regex::new(r"oai_citation").expect("valid regex"),
        Regex::new(r"contentReference\[").expect("valid regex"),
        Regex::new(r"utm_source=(chatgpt\.com|openai|copilot\.com)").expect("valid regex"),
        Regex::new(r"referrer=grok\.com").expect("valid regex"),
        Regex::new(r"(?i)as an ai language model").expect("valid regex"),
        Regex::new(r"(?i)as a large language model").expect("valid regex"),
        Regex::new(r"attributableIndex").expect("valid regex"),
        Regex::new(r"grok.card").expect("valid regex"),
    ];

    static ref COPULA_AVOIDANCE: HashSet<&'static str> = [
        "serves as a", "serves as an", "serves as the",
        "stands as a", "stands as an", "stands as the",
        "marks a", "marks an", "marks the",
        "represents a", "represents an", "represents the",
        "features a", "features an", "features the",
        "offers a", "offers an", "offers the",
        "boasts a", "boasts an", "boasts the",
        "holds the distinction", "ventured into",
    ].into_iter().collect();

    static ref NOTABILITY_EMPHASIS: HashSet<&'static str> = [
        "independent coverage", "media outlets",
        "active social media presence", "social media presence",
        "has been featured in", "was featured in",
        "has been profiled in", "was profiled in",
        "has been cited in", "has been mentioned in",
        "regional media", "national media", "local media",
        "prominent media", "various publications",
        "written by a leading", "maintained a strong digital presence",
    ].into_iter().collect();

    static ref OUTLINE_CONCLUSION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)despite (its|their|these) .{5,40}, .{5,40} faces? (several |numerous )?challenges").expect("valid regex"),
        Regex::new(r"(?i)despite these challenges,").expect("valid regex"),
        Regex::new(r"(?im)^#+\s*(future (outlook|prospects|directions)|challenges and)").expect("valid regex"),
        Regex::new(r"(?i)the future of .{5,40} lies in").expect("valid regex"),
        Regex::new(r"(?i)continues to (evolve|thrive|grow) in response").expect("valid regex"),
    ];

    static ref EMOJI_REGEX: Regex = Regex::new(
        r"[\x{1F300}-\x{1F9FF}\x{2600}-\x{26FF}\x{2700}-\x{27BF}\x{FE00}-\x{FE0F}\x{1FA00}-\x{1FA6F}\x{1FA70}-\x{1FAFF}]"
    ).expect("valid regex");
}

pub fn check_ai_vocabulary(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for word in AI_VOCAB.iter() {
        if lower_text.contains(word) {
            count += lower_text.matches(word).count();
            flagged.push(word.to_string());
        }
    }

    (count, flagged)
}

pub fn check_conversational_filler(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for phrase in CONVERSATIONAL_FILLER.iter() {
        if lower_text.contains(phrase) {
            count += lower_text.matches(phrase).count();
            flagged.push(phrase.to_string());
        }
    }

    (count, flagged)
}

pub fn check_ai_self_reference(text: &str) -> usize {
    let mut count = 0;
    for pattern in AI_SELF_REFERENCE.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_tends_to_patterns(text: &str) -> usize {
    let mut count = 0;
    for pattern in TENDS_TO_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_bullet_points(text: &str) -> usize {
    BULLET_POINT_REGEX.find_iter(text).count()
}

pub fn check_contrast_patterns(text: &str) -> usize {
    let mut count = 0;
    for pattern in CONTRAST_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count.min(10)
}

pub fn check_generalizations(text: &str) -> usize {
    let mut count = 0;
    for pattern in GENERALIZATION_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_philosophical_patterns(text: &str) -> usize {
    let mut count = 0;
    for pattern in PHILOSOPHICAL_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_self_referential(text: &str) -> usize {
    let mut count = 0;
    for pattern in SELF_REFERENTIAL.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_sentence_fragments(text: &str) -> usize {
    lazy_static::lazy_static! {
        static ref FRAGMENT_PATTERNS: Vec<Regex> = vec![
            Regex::new(r"(?im)^[A-Z][a-z]+\.$").expect("valid regex"),
            Regex::new(r"(?im)^Not \w+\. \w+\.$").expect("valid regex"),
        ];
    }

    let mut count = 0;
    for pattern in FRAGMENT_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_comma_density(text: &str) -> (usize, usize) {
    let word_count = text.split_whitespace().count();
    let comma_count = text.matches(',').count();

    if word_count == 0 {
        return (0, 0);
    }

    let comma_ratio = comma_count * 100 / word_count;
    (comma_count, comma_ratio)
}

pub fn check_negative_parallelisms(text: &str) -> (usize, Vec<String>) {
    let mut count = 0;
    let mut flagged = Vec::new();

    for pattern in NEGATIVE_PARALLELISM_PATTERNS.iter() {
        for capture in pattern.find_iter(text) {
            count += 1;
            flagged.push(capture.as_str().to_string());
        }
    }

    (count, flagged)
}

pub fn check_rule_of_three(text: &str) -> (usize, Vec<String>) {
    let mut count = 0;
    let mut flagged = Vec::new();

    let list_pattern = Regex::new(r"(\w+),\s*(\w+),\s+and\s+(\w+)").expect("valid regex");
    for capture in list_pattern.find_iter(text) {
        let matched = capture.as_str();
        if matched.len() < 50 {
            count += 1;
            flagged.push(matched.to_string());
        }
    }

    let adjective_pattern =
        Regex::new(r"\b(\w+),?\s+(\w+),?\s+and\s+(\w+)\s+\w+\b").expect("valid regex");
    for capture in adjective_pattern.find_iter(text) {
        count += 1;
        flagged.push(capture.as_str().to_string());
    }

    (count, flagged)
}

pub fn check_promotional_language(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for phrase in PROMOTIONAL_WORDS.iter() {
        if lower_text.contains(phrase) {
            count += lower_text.matches(phrase).count();
            flagged.push(phrase.to_string());
        }
    }

    let hype_words = [
        "exciting",
        "innovative",
        "cutting-edge",
        "state-of-the-art",
        "revolutionary",
    ];
    for word in hype_words.iter() {
        let matches = lower_text.matches(word).count();
        if matches > 0 {
            count += matches;
            flagged.push(word.to_string());
        }
    }

    (count, flagged)
}

pub fn check_significance_emphasis(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for phrase in SIGNIFICANCE_PHRASES.iter() {
        if lower_text.contains(phrase) {
            count += lower_text.matches(phrase).count();
            flagged.push(phrase.to_string());
        }
    }

    (count, flagged)
}

pub fn check_vague_attributions(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for phrase in VAGUE_ATTRIBUTIONS.iter() {
        if lower_text.contains(phrase) {
            count += lower_text.matches(phrase).count();
            flagged.push(phrase.to_string());
        }
    }

    (count, flagged)
}

pub fn check_formatting_artifacts(text: &str) -> (usize, Vec<String>) {
    let mut count = 0;
    let mut flagged = Vec::new();

    if EXCESSIVE_BOLD_REGEX.is_match(text) {
        count += 1;
        flagged.push("Excessive bold formatting".to_string());
    }

    let em_dash_count = text.matches('\u{2014}').count(); // \u{2014} = em dash (—)
    if em_dash_count > 2 {
        count += 1;
        flagged.push(format!(
            "Multiple em dashes ({} occurrences)",
            em_dash_count
        ));
    }

    let header_count = MARKDOWN_HEADER_REGEX.find_iter(text).count();
    if header_count > 0 {
        count += header_count;
        flagged.push(format!("Markdown headers ({} found)", header_count));
    }

    if text.contains("```") {
        count += 1;
        flagged.push("Markdown code block detected".to_string());
    }

    if MARKDOWN_BOLD_INLINE_REGEX.is_match(text) {
        count += 1;
        flagged.push("Markdown bold in prose".to_string());
    }

    (count, flagged)
}

pub fn check_curly_quotes(text: &str) -> bool {
    text.contains('\u{201c}') // left double quote (\u{201c})
        || text.contains('\u{201d}') // right double quote (\u{201d})
        || text.contains("\u{2018}") // left single quote (\u{2018})
        || text.contains("\u{2019}") // right single quote (\u{2019})
}

pub fn check_knowledge_cutoff(text: &str) -> bool {
    for pattern in KNOWLEDGE_CUTOFF_PATTERNS.iter() {
        if pattern.is_match(text) {
            return true;
        }
    }
    false
}

pub fn check_despite_pattern(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.starts_with("despite its") || lower.contains("despite these challenges")
}

pub fn check_didactic_disclaimers(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for phrase in DIDACTIC_DISCLAIMERS.iter() {
        let matches = lower_text.matches(phrase).count();
        if matches > 0 {
            count += matches;
            flagged.push(phrase.to_string());
        }
    }

    (count, flagged)
}

pub fn check_collaborative_comm(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for phrase in COLLABORATIVE_COMM.iter() {
        let matches = lower_text.matches(phrase).count();
        if matches > 0 {
            count += matches;
            flagged.push(phrase.to_string());
        }
    }

    (count, flagged)
}

pub fn check_superficial_analyses(text: &str) -> usize {
    let mut count = 0;
    for pattern in SUPERFICIAL_ING_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_section_summaries(text: &str) -> usize {
    let mut count = 0;
    for pattern in SECTION_SUMMARY_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_emoji_usage(text: &str) -> usize {
    EMOJI_REGEX.find_iter(text).count()
}

pub fn check_placeholder_text(text: &str) -> usize {
    let mut count = 0;
    for pattern in PLACEHOLDER_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_chatgpt_artifacts(text: &str) -> usize {
    let mut count = 0;
    for pattern in CHATGPT_ARTIFACT_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn check_copula_avoidance(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for phrase in COPULA_AVOIDANCE.iter() {
        let matches = lower_text.matches(phrase).count();
        if matches > 0 {
            count += matches;
            flagged.push(phrase.to_string());
        }
    }

    (count, flagged)
}

pub fn check_notability_emphasis(text: &str) -> (usize, Vec<String>) {
    let lower_text = text.to_lowercase();
    let mut count = 0;
    let mut flagged = Vec::new();

    for phrase in NOTABILITY_EMPHASIS.iter() {
        let matches = lower_text.matches(phrase).count();
        if matches > 0 {
            count += matches;
            flagged.push(phrase.to_string());
        }
    }

    (count, flagged)
}

pub fn check_outline_conclusions(text: &str) -> usize {
    let mut count = 0;
    for pattern in OUTLINE_CONCLUSION_PATTERNS.iter() {
        count += pattern.find_iter(text).count();
    }
    count
}

pub fn analyze_indicators(text: &str) -> (IndicatorBreakdown, Vec<String>) {
    let mut breakdown = IndicatorBreakdown::new();
    let mut all_flagged: Vec<String> = Vec::new();

    let (count, flagged) = check_ai_vocabulary(text);
    breakdown.ai_vocabulary = count;
    all_flagged.extend(flagged);

    let (count, flagged) = check_negative_parallelisms(text);
    breakdown.negative_parallelisms = count;
    all_flagged.extend(flagged);

    let (count, flagged) = check_rule_of_three(text);
    breakdown.rule_of_three = count;
    all_flagged.extend(flagged);

    let (count, flagged) = check_promotional_language(text);
    breakdown.promotional_phrases = count;
    all_flagged.extend(flagged);

    let (count, flagged) = check_significance_emphasis(text);
    breakdown.significance_emphasis = count;
    all_flagged.extend(flagged);

    let (count, flagged) = check_vague_attributions(text);
    breakdown.vague_attributions = count;
    all_flagged.extend(flagged);

    let (count, flagged) = check_formatting_artifacts(text);
    breakdown.formatting_artifacts = count;
    all_flagged.extend(flagged);

    breakdown.curly_quotes = check_curly_quotes(text);
    if breakdown.curly_quotes {
        all_flagged.push("Curly quotation marks/apostrophes detected".to_string());
    }

    breakdown.knowledge_cutoff_disclaimer = check_knowledge_cutoff(text);
    if breakdown.knowledge_cutoff_disclaimer {
        all_flagged.push("Knowledge-cutoff disclaimer detected".to_string());
    }

    if check_despite_pattern(text) {
        all_flagged.push("'Despite its...' pattern detected".to_string());
    }

    let (count, flagged) = check_conversational_filler(text);
    breakdown.conversational_filler = count;
    all_flagged.extend(flagged);

    breakdown.ai_self_reference = check_ai_self_reference(text);
    if breakdown.ai_self_reference > 0 {
        all_flagged.push(format!(
            "AI self-reference patterns ({} occurrences)",
            breakdown.ai_self_reference
        ));
    }

    breakdown.tends_to_patterns = check_tends_to_patterns(text);
    if breakdown.tends_to_patterns > 0 {
        all_flagged.push(format!(
            "\"Tends to\" patterns ({} occurrences)",
            breakdown.tends_to_patterns
        ));
    }

    breakdown.bullet_points = check_bullet_points(text);
    if breakdown.bullet_points > 0 {
        all_flagged.push(format!("Bullet points ({} items)", breakdown.bullet_points));
    }

    breakdown.contrast_patterns = check_contrast_patterns(text);
    if breakdown.contrast_patterns > 0 {
        all_flagged.push(format!(
            "Contrast patterns ({} occurrences)",
            breakdown.contrast_patterns
        ));
    }

    breakdown.generalizations = check_generalizations(text);
    if breakdown.generalizations > 0 {
        all_flagged.push(format!(
            "Generalizations about AI/humans ({} occurrences)",
            breakdown.generalizations
        ));
    }

    breakdown.philosophical_patterns = check_philosophical_patterns(text);
    if breakdown.philosophical_patterns > 0 {
        all_flagged.push(format!(
            "Philosophical/metaphorical patterns ({} occurrences)",
            breakdown.philosophical_patterns
        ));
    }

    breakdown.self_referential = check_self_referential(text);
    if breakdown.self_referential > 0 {
        all_flagged.push(format!(
            "Self-referential statements ({} occurrences)",
            breakdown.self_referential
        ));
    }

    breakdown.sentence_fragments = check_sentence_fragments(text);
    if breakdown.sentence_fragments > 0 {
        all_flagged.push(format!(
            "Sentence fragments ({} occurrences)",
            breakdown.sentence_fragments
        ));
    }

    let (_commas, comma_ratio) = check_comma_density(text);
    breakdown.high_comma_density = comma_ratio > 10;
    if breakdown.high_comma_density {
        all_flagged.push(format!("High comma density ({}% of words)", comma_ratio));
    }

    let (count, flagged) = check_didactic_disclaimers(text);
    breakdown.didactic_disclaimers = count;
    all_flagged.extend(flagged);

    let (count, flagged) = check_collaborative_comm(text);
    breakdown.collaborative_comm = count;
    all_flagged.extend(flagged);

    breakdown.superficial_analyses = check_superficial_analyses(text);
    if breakdown.superficial_analyses > 0 {
        all_flagged.push(format!(
            "Superficial -ing analyses ({} occurrences)",
            breakdown.superficial_analyses
        ));
    }

    breakdown.section_summaries = check_section_summaries(text);
    if breakdown.section_summaries > 0 {
        all_flagged.push(format!(
            "Section summary phrases ({} occurrences)",
            breakdown.section_summaries
        ));
    }

    breakdown.emoji_usage = check_emoji_usage(text);
    if breakdown.emoji_usage > 0 {
        all_flagged.push(format!("Emoji usage ({} found)", breakdown.emoji_usage));
    }

    breakdown.placeholder_text = check_placeholder_text(text);
    if breakdown.placeholder_text > 0 {
        all_flagged.push(format!(
            "Placeholder text ({} occurrences)",
            breakdown.placeholder_text
        ));
    }

    breakdown.chatgpt_artifacts = check_chatgpt_artifacts(text);
    if breakdown.chatgpt_artifacts > 0 {
        all_flagged.push(format!(
            "ChatGPT artifacts ({} occurrences)",
            breakdown.chatgpt_artifacts
        ));
    }

    let (count, flagged) = check_copula_avoidance(text);
    breakdown.copula_avoidance = count;
    all_flagged.extend(flagged);

    let (count, flagged) = check_notability_emphasis(text);
    breakdown.notability_emphasis = count;
    all_flagged.extend(flagged);

    breakdown.outline_conclusions = check_outline_conclusions(text);
    if breakdown.outline_conclusions > 0 {
        all_flagged.push(format!(
            "Outline-like conclusions ({} occurrences)",
            breakdown.outline_conclusions
        ));
    }

    (breakdown, all_flagged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_vocabulary_detection() {
        let text = "The intricate tapestry of history showcases pivotal moments in our evolving landscape.";
        let (count, _) = check_ai_vocabulary(text);
        assert!(count >= 3);
    }

    #[test]
    fn test_negative_parallelism() {
        let text = "It's not just a car, it's a lifestyle. Not only fast, but also beautiful.";
        let (count, _) = check_negative_parallelisms(text);
        assert!(count >= 2);
    }

    #[test]
    fn test_curly_quotes() {
        let text = "This is a \u{201c}test\u{201d} with \u{2018}curly quotes\u{2019}.";
        assert!(check_curly_quotes(text));
    }

    #[test]
    fn test_knowledge_cutoff() {
        let text = "As of my last knowledge update in January 2022, I don't have that information.";
        assert!(check_knowledge_cutoff(text));
    }

    #[test]
    fn test_didactic_disclaimers() {
        let text = "It's important to note that this approach has limitations. It is worth noting that the results may vary depending on the context.";
        let (count, _) = check_didactic_disclaimers(text);
        assert!(count >= 2);
    }

    #[test]
    fn test_collaborative_comm() {
        let text = "I hope this helps! Would you like me to explain further? Feel free to ask.";
        let (count, _) = check_collaborative_comm(text);
        assert!(count >= 2);
    }

    #[test]
    fn test_superficial_analyses() {
        let text = "The temple was built in 1823, highlighting its historical significance. The river flows through the valley, contributing to the region's biodiversity.";
        let count = check_superficial_analyses(text);
        assert!(count >= 2);
    }

    #[test]
    fn test_section_summaries() {
        let text = "In conclusion, the Renaissance was a pivotal period.\nOverall, the results demonstrate significant improvement.";
        let count = check_section_summaries(text);
        assert!(count >= 2);
    }

    #[test]
    fn test_emoji_detection() {
        let text = "\u{1F9E0} Cognitive Patterns:\n\u{1F6A8} Key Warning:";
        let count = check_emoji_usage(text);
        assert!(count >= 2);
    }

    #[test]
    fn test_placeholder_text() {
        let text = "[insert source here] and PASTE_URL_HERE with date 2025-XX-XX";
        let count = check_placeholder_text(text);
        assert!(count >= 3);
    }

    #[test]
    fn test_chatgpt_artifacts() {
        let text = "Some text turn0search0 and oaicite:1 with utm_source=chatgpt.com link";
        let count = check_chatgpt_artifacts(text);
        assert!(count >= 3);
    }

    #[test]
    fn test_copula_avoidance() {
        let text = "The building serves as a community center. It features a large auditorium. The park offers a peaceful retreat.";
        let (count, _) = check_copula_avoidance(text);
        assert!(count >= 3);
    }

    #[test]
    fn test_outline_conclusions() {
        let text = "Despite its industrial prosperity, the city faces several challenges including pollution.";
        let count = check_outline_conclusions(text);
        assert!(count >= 1);
    }
}
