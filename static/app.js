import init, { analyze_text } from '../pkg/vyasa.js';

let wasmReady = false;
init().then(() => { wasmReady = true; }).catch(e => {
    console.error('WASM init failed, falling back to API:', e);
});

const DOM = {
    textInput: document.getElementById('textInput'),
    analyzeBtn: document.getElementById('analyzeBtn'),
    clearBtn: document.getElementById('clearBtn'),
    sampleBtn: document.getElementById('sampleBtn'),
    charLimit: document.getElementById('charLimit'),
    charLimitLabel: document.getElementById('charLimitLabel'),
    charCount: document.getElementById('charCount'),
    loading: document.getElementById('loading'),
    resultsSection: document.getElementById('resultsSection'),
    summaryText: document.getElementById('summaryText'),
    wordCount: document.getElementById('wordCount'),
    patternCount: document.getElementById('patternCount'),
    flaggedCount: document.getElementById('flaggedCount'),
    highlightedText: document.getElementById('highlightedText'),
    breakdownBody: document.getElementById('breakdownBody'),
    flaggedSection: document.getElementById('flaggedSection'),
    flaggedList: document.getElementById('flaggedList')
};

const SAMPLE_TEXT = `The intricate tapestry of Renaissance art showcases the pivotal role that patronage played in fostering creative expression. Additionally, the meticulous craftsmanship of artists such as Leonardo da Vinci underscores the enduring legacy of this transformative period.

Not only did the Renaissance revolutionize artistic techniques, but it also bolstered a comprehensive understanding of human anatomy. The interplay between science and art during this era stands as a testament to the holistic approach that characterized the movement, highlighting its lasting influence on Western civilization.

Experts argue that the Renaissance represents a crucial turning point in Western civilization, seamlessly blending innovation with tradition. This groundbreaking period continues to garner attention from scholars and enthusiasts alike, contributing to the cultural landscape. It's important to note that the movement's impact extends far beyond the visual arts, encompassing literature, science, and philosophy.

In conclusion, the Renaissance serves as a reminder of humanity's capacity for creative and intellectual achievement, setting the stage for the modern world.`;

// Highlight category → human-readable label, used for hover tooltips on the
// rendered spans. The categories themselves are emitted by the Rust
// `collect_highlights` function so that the wordlists/patterns live in one
// place only. Keep this map in sync with the categories produced there.
const CATEGORY_LABELS = {
    vocab: 'AI vocabulary',
    promo: 'Promotional language',
    sig: 'Significance / legacy emphasis',
    didactic: 'Didactic disclaimer',
    collab: 'Collaborative communication',
    filler: 'Conversational filler',
    vague: 'Vague attribution',
    copula: 'Copula avoidance',
    notability: 'Notability emphasis',
    'self-ref': 'Self-reference / AI self-reference',
    philosophical: 'Philosophical pattern',
    tendsto: '"Tends to" pattern',
    contrast: 'Contrast / hedging',
    general: 'Generalization about AI/humans',
    structure: 'Negative-parallelism structure',
    superficial: 'Superficial -ing analysis',
    section: 'Section summary',
    placeholder: 'Placeholder text',
    chatgpt: 'ChatGPT artifact',
    cutoff: 'Knowledge-cutoff disclaimer',
    outline: 'Outline-like conclusion',
    emoji: 'Emoji'
};

DOM.analyzeBtn.addEventListener('click', analyzeText);
DOM.clearBtn.addEventListener('click', clearText);
DOM.sampleBtn.addEventListener('click', loadSample);

DOM.textInput.addEventListener('keydown', function(e) {
    if (e.ctrlKey && e.key === 'Enter') {
        analyzeText();
    }
});

function formatCharLimit(value) {
    if (value >= 1000000) return (value / 1000000).toFixed(1).replace(/\.0$/, '') + 'M chars';
    return (value / 1000).toFixed(0) + 'K chars';
}

DOM.charLimit.addEventListener('input', function() {
    DOM.charLimitLabel.textContent = formatCharLimit(parseInt(this.value));
});

DOM.textInput.addEventListener('input', updateCharCount);
function updateCharCount() {
    const length = DOM.textInput.value.length;
    const limit = parseInt(DOM.charLimit.value);
    if (length > 0) {
        const percentage = ((length / limit) * 100).toFixed(0);
        DOM.charCount.textContent = `(${length.toLocaleString()} / ${limit.toLocaleString()} chars, ${percentage}%)`;
        DOM.charCount.style.color = length > limit ? '#d32f2f' : '#999';
    } else {
        DOM.charCount.textContent = '';
    }
}

async function analyzeText() {
    const text = DOM.textInput.value.trim();
    if (!text) {
        alert('Please enter some text to analyze.');
        return;
    }

    const maxChars = parseInt(DOM.charLimit.value);

    if (text.length > maxChars) {
        alert(`Text is ${text.length.toLocaleString()} characters, which exceeds your limit of ${maxChars.toLocaleString()}. Increase the slider or shorten the text.`);
        return;
    }

    DOM.analyzeBtn.disabled = true;
    DOM.loading.textContent = text.length > 500000 ? 'Analyzing (large text, this may take a moment)...' : 'Analyzing...';
    DOM.loading.style.display = 'block';
    DOM.resultsSection.style.display = 'none';

    try {
        let result;

        if (wasmReady) {
            result = JSON.parse(analyze_text(text, maxChars));
        } else {
            const response = await fetch('/api/detect', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ text, max_chars: maxChars })
            });
            if (!response.ok) throw new Error(`Server error: ${response.status}`);
            result = await response.json();
        }

        if (result.error) {
            alert(result.error);
            return;
        }

        displayResults(result, text);
    } catch (error) {
        console.error('Error:', error);
        alert('Failed to analyze text. Please try again.');
    } finally {
        DOM.analyzeBtn.disabled = false;
        DOM.loading.style.display = 'none';
    }
}

function displayResults(result, originalText) {
    const { breakdown, flagged_phrases, highlights, word_count } = result;

    DOM.resultsSection.style.display = 'block';
    DOM.wordCount.textContent = word_count;

    const patternTypes = getDetectedPatterns(breakdown);
    DOM.patternCount.textContent = patternTypes.length;
    DOM.flaggedCount.textContent = flagged_phrases ? flagged_phrases.length : 0;

    generateSummary(patternTypes, word_count, breakdown);
    highlightText(originalText, highlights || []);
    populateBreakdown(breakdown);
    populateFlaggedPhrases(flagged_phrases);

    DOM.resultsSection.scrollIntoView({ behavior: 'smooth' });
}

function getDetectedPatterns(breakdown) {
    const patterns = [];
    if (breakdown.chatgpt_artifacts > 0) patterns.push({ name: 'ChatGPT artifacts', count: breakdown.chatgpt_artifacts, severity: 'definitive' });
    if (breakdown.placeholder_text > 0) patterns.push({ name: 'placeholder text', count: breakdown.placeholder_text, severity: 'definitive' });
    if (breakdown.knowledge_cutoff_disclaimer) patterns.push({ name: 'knowledge-cutoff disclaimer', count: 1, severity: 'definitive' });
    if (breakdown.collaborative_comm > 0) patterns.push({ name: 'collaborative communication', count: breakdown.collaborative_comm, severity: 'strong' });
    if (breakdown.self_referential > 0) patterns.push({ name: 'self-referential statements', count: breakdown.self_referential, severity: 'strong' });
    if (breakdown.ai_self_reference > 0) patterns.push({ name: 'AI self-reference', count: breakdown.ai_self_reference, severity: 'strong' });
    if (breakdown.superficial_analyses > 0) patterns.push({ name: 'superficial -ing analyses', count: breakdown.superficial_analyses, severity: 'strong' });
    if (breakdown.outline_conclusions > 0) patterns.push({ name: 'outline-like conclusions', count: breakdown.outline_conclusions, severity: 'strong' });
    if (breakdown.ai_vocabulary > 0) patterns.push({ name: 'AI vocabulary', count: breakdown.ai_vocabulary, severity: 'moderate' });
    if (breakdown.significance_emphasis > 0) patterns.push({ name: 'significance/legacy emphasis', count: breakdown.significance_emphasis, severity: 'moderate' });
    if (breakdown.copula_avoidance > 0) patterns.push({ name: 'copula avoidance', count: breakdown.copula_avoidance, severity: 'moderate' });
    if (breakdown.negative_parallelisms > 0) patterns.push({ name: 'negative parallelisms', count: breakdown.negative_parallelisms, severity: 'moderate' });
    if (breakdown.promotional_phrases > 0) patterns.push({ name: 'promotional language', count: breakdown.promotional_phrases, severity: 'moderate' });
    if (breakdown.didactic_disclaimers > 0) patterns.push({ name: 'didactic disclaimers', count: breakdown.didactic_disclaimers, severity: 'moderate' });
    if (breakdown.vague_attributions > 0) patterns.push({ name: 'vague attributions', count: breakdown.vague_attributions, severity: 'moderate' });
    if (breakdown.notability_emphasis > 0) patterns.push({ name: 'notability emphasis', count: breakdown.notability_emphasis, severity: 'moderate' });
    if (breakdown.section_summaries > 0) patterns.push({ name: 'section summaries', count: breakdown.section_summaries, severity: 'moderate' });
    if (breakdown.rule_of_three > 0) patterns.push({ name: 'rule-of-three lists', count: breakdown.rule_of_three, severity: 'mild' });
    if (breakdown.formatting_artifacts > 0) patterns.push({ name: 'formatting artifacts', count: breakdown.formatting_artifacts, severity: 'mild' });
    if (breakdown.emoji_usage > 0) patterns.push({ name: 'emoji usage', count: breakdown.emoji_usage, severity: 'mild' });
    if (breakdown.philosophical_patterns > 0) patterns.push({ name: 'philosophical patterns', count: breakdown.philosophical_patterns, severity: 'mild' });
    if (breakdown.generalizations > 0) patterns.push({ name: 'generalizations', count: breakdown.generalizations, severity: 'mild' });
    if (breakdown.conversational_filler > 0) patterns.push({ name: 'conversational filler', count: breakdown.conversational_filler, severity: 'mild' });
    if (breakdown.tends_to_patterns > 0) patterns.push({ name: '"tends to" patterns', count: breakdown.tends_to_patterns, severity: 'mild' });
    if (breakdown.bullet_points > 0) patterns.push({ name: 'bullet points', count: breakdown.bullet_points, severity: 'mild' });
    if (breakdown.contrast_patterns > 0) patterns.push({ name: 'contrast/hedging', count: breakdown.contrast_patterns, severity: 'mild' });
    if (breakdown.sentence_fragments > 0) patterns.push({ name: 'sentence fragments', count: breakdown.sentence_fragments, severity: 'mild' });
    if (breakdown.high_comma_density) patterns.push({ name: 'high comma density', count: 1, severity: 'mild' });
    if (breakdown.curly_quotes) patterns.push({ name: 'curly quotes', count: 1, severity: 'mild' });
    return patterns;
}

function generateSummary(patternTypes, wordCount, breakdown) {
    if (patternTypes.length === 0) {
        DOM.summaryText.textContent = 'No common AI writing patterns were detected in this text.';
        return;
    }

    const definitive = patternTypes.filter(pattern => pattern.severity === 'definitive');
    const strong = patternTypes.filter(pattern => pattern.severity === 'strong');

    let summary = '';

    if (definitive.length > 0) {
        summary = `Found definitive AI indicators: ${definitive.map(pattern => pattern.name).join(', ')}. `;
        summary += `These are artifacts that only appear in AI-generated text. `;
    } else if (strong.length > 0 && patternTypes.length > 5) {
        const notable = patternTypes.slice(0, 5).map(pattern => pattern.name);
        summary = `Found ${patternTypes.length} types of AI writing patterns across ${wordCount} words, including strong indicators: ${notable.join(', ')}. `;
    } else {
        const notable = patternTypes.slice(0, 4).map(pattern => pattern.name);
        summary = `Found ${patternTypes.length} types of AI writing patterns across ${wordCount} words. Notable: ${notable.join(', ')}. `;
    }

    const totalHits = patternTypes.reduce((sum, pattern) => sum + pattern.count, 0);
    if (definitive.length === 0) {
        if (totalHits > 15) {
            summary += `High density of patterns detected. `;
        } else if (totalHits > 8) {
            summary += `Several instances found. `;
        } else {
            summary += `Only a few instances found - these can occur in human writing too. `;
        }
    }

    summary += `Review the highlights and decide for yourself.`;
    DOM.summaryText.textContent = summary;
}

// Severity priority for resolving overlaps when two patterns claim the same
// span — definitive/strong indicators win over moderate/mild ones so the
// reader sees the most informative label.
const HIGHLIGHT_PRIORITY = {
    chatgpt: 1,
    placeholder: 2,
    cutoff: 3,
    'self-ref': 4,
    collab: 5,
    didactic: 6,
    superficial: 7,
    sig: 8,
    copula: 9,
    notability: 10,
    vocab: 11,
    promo: 12,
    vague: 13,
    filler: 14,
    outline: 15,
    section: 16,
    structure: 17,
    contrast: 18,
    general: 19,
    tendsto: 20,
    philosophical: 21,
    emoji: 22
};

function mergeHighlights(highlights) {
    const sorted = highlights.slice().sort((a, b) => {
        const priorityA = HIGHLIGHT_PRIORITY[a.category] || 99;
        const priorityB = HIGHLIGHT_PRIORITY[b.category] || 99;
        if (priorityA !== priorityB) return priorityA - priorityB;
        if (a.start !== b.start) return a.start - b.start;
        return (b.end - b.start) - (a.end - a.start);
    });

    const claimed = [];
    const result = [];

    for (const highlight of sorted) {
        if (highlight.end <= highlight.start) continue;

        let segments = [{ start: highlight.start, end: highlight.end }];
        for (const claim of claimed) {
            const next = [];
            for (const segment of segments) {
                if (segment.end <= claim.start || segment.start >= claim.end) {
                    next.push(segment);
                } else {
                    if (segment.start < claim.start) {
                        next.push({ start: segment.start, end: claim.start });
                    }
                    if (segment.end > claim.end) {
                        next.push({ start: claim.end, end: segment.end });
                    }
                }
            }
            segments = next;
            if (segments.length === 0) break;
        }

        for (const segment of segments) {
            // Skip slivers that would render as empty/blank spans after
            // overlap subtraction (e.g. a single trailing comma).
            if (segment.end - segment.start < 1) continue;
            result.push({
                start: segment.start,
                end: segment.end,
                category: highlight.category
            });
            claimed.push({ start: segment.start, end: segment.end });
        }
    }

    result.sort((a, b) => a.start - b.start);
    return result;
}

function highlightText(text, highlights) {
    if (!highlights || highlights.length === 0) {
        DOM.highlightedText.textContent = text;
        return;
    }

    const merged = mergeHighlights(highlights);

    let html = '';
    let position = 0;
    for (const highlight of merged) {
        if (highlight.start < position) continue;
        if (highlight.start > position) {
            html += escapeHtml(text.substring(position, highlight.start));
        }
        const label = CATEGORY_LABELS[highlight.category] || highlight.category;
        const slice = text.substring(highlight.start, highlight.end);
        html += `<span class="highlight highlight-${highlight.category}" title="${escapeHtml(label)}">${escapeHtml(slice)}</span>`;
        position = highlight.end;
    }
    if (position < text.length) {
        html += escapeHtml(text.substring(position));
    }

    DOM.highlightedText.innerHTML = html;
}

function populateBreakdown(breakdown) {
    const rows = [
        { name: 'ChatGPT Artifacts', count: breakdown.chatgpt_artifacts, desc: 'turn0search, oaicite, utm_source=chatgpt - definitive proof of AI tool use', severity: 'definitive' },
        { name: 'Placeholder Text', count: breakdown.placeholder_text, desc: '[insert X here], PASTE_URL_HERE, XX-XX dates - unfilled AI templates', severity: 'definitive' },
        { name: 'Knowledge Cutoff', count: breakdown.knowledge_cutoff_disclaimer ? 1 : 0, desc: '"As of my last update" - direct AI self-disclosure', severity: 'definitive' },
        { name: 'Collaborative Comm', count: breakdown.collaborative_comm, desc: '"I hope this helps", "Would you like", "Certainly!" - AI talking to user', severity: 'strong' },
        { name: 'Self-Referential', count: breakdown.self_referential, desc: '"I don\'t remember", "I have no body" - AI talking about being AI', severity: 'strong' },
        { name: 'AI Self-Reference', count: breakdown.ai_self_reference, desc: 'Text explicitly discussing "AI" vs "humans"', severity: 'strong' },
        { name: 'Superficial Analyses', count: breakdown.superficial_analyses, desc: '", highlighting its...", ", underscoring the..." - tacked-on -ing phrases', severity: 'strong' },
        { name: 'Outline Conclusions', count: breakdown.outline_conclusions, desc: '"Despite its... faces challenges" - formulaic conclusion pattern', severity: 'strong' },
        { name: 'AI Vocabulary', count: breakdown.ai_vocabulary, desc: 'delve, tapestry, pivotal, leverage, comprehensive, robust, seamless...', severity: 'moderate' },
        { name: 'Significance/Legacy', count: breakdown.significance_emphasis, desc: '"Testament to", "pivotal role", "enduring legacy" - over-emphasizing importance', severity: 'moderate' },
        { name: 'Copula Avoidance', count: breakdown.copula_avoidance, desc: '"Serves as" instead of "is", "features" instead of "has"', severity: 'moderate' },
        { name: 'Negative Parallelisms', count: breakdown.negative_parallelisms, desc: '"Not just X, but Y" - AI overuses this balanced structure', severity: 'moderate' },
        { name: 'Promotional Language', count: breakdown.promotional_phrases, desc: 'Showcasing, boasts a, seamlessly, world-class - hype words', severity: 'moderate' },
        { name: 'Didactic Disclaimers', count: breakdown.didactic_disclaimers, desc: '"It\'s important to note", "worth noting" - AI hedging', severity: 'moderate' },
        { name: 'Vague Attributions', count: breakdown.vague_attributions, desc: '"Experts say", "observers note" - avoiding specific sources', severity: 'moderate' },
        { name: 'Notability Emphasis', count: breakdown.notability_emphasis, desc: '"Independent coverage", "media outlets", "featured in" - asserting importance', severity: 'moderate' },
        { name: 'Section Summaries', count: breakdown.section_summaries, desc: '"In summary", "In conclusion" - AI wrapping-up habit', severity: 'moderate' },
        { name: 'Rule of Three', count: breakdown.rule_of_three, desc: '"X, Y, and Z" - AI defaults to three-item lists', severity: 'mild' },
        { name: 'Formatting Issues', count: breakdown.formatting_artifacts, desc: 'Markdown headers, excessive bold, em dashes, code blocks', severity: 'mild' },
        { name: 'Emoji Usage', count: breakdown.emoji_usage, desc: 'AI chatbots often decorate headings/bullets with emoji', severity: 'mild' },
        { name: 'Philosophical', count: breakdown.philosophical_patterns, desc: 'Abstract metaphors and poetic AI-style reasoning', severity: 'mild' },
        { name: 'Generalizations', count: breakdown.generalizations, desc: '"AI always", "humans never" - sweeping claims', severity: 'mild' },
        { name: 'Conversational Filler', count: breakdown.conversational_filler, desc: '"Let me break this down", "Here\'s the thing"', severity: 'mild' },
        { name: '"Tends To"', count: breakdown.tends_to_patterns, desc: '"X tends to Y" - classic AI explanation style', severity: 'mild' },
        { name: 'Bullet Points', count: breakdown.bullet_points, desc: 'Structured lists - AI defaults to bullets heavily', severity: 'mild' },
        { name: 'Contrast Patterns', count: breakdown.contrast_patterns, desc: '"While X, they also Y" - balanced hedging', severity: 'mild' },
        { name: 'Sentence Fragments', count: breakdown.sentence_fragments, desc: '"Not facts. Texture." - short dramatic fragments', severity: 'mild' },
        { name: 'Comma Density', count: breakdown.high_comma_density ? 1 : 0, desc: 'Unusually high comma usage - over-structured sentences', severity: 'mild' },
        { name: 'Curly Quotes', count: breakdown.curly_quotes ? 1 : 0, desc: 'Smart quotes from ChatGPT/DeepSeek (not Gemini/Claude)', severity: 'mild' },
    ];

    const filteredRows = rows.filter(row => row.count > 0);

    if (filteredRows.length === 0) {
        DOM.breakdownBody.innerHTML = '<tr><td colspan="3" style="text-align:center; color:#888;">No AI patterns detected</td></tr>';
        return;
    }

    const severityOrder = { definitive: 0, strong: 1, moderate: 2, mild: 3 };
    const severityColor = { definitive: '#d32f2f', strong: '#e65100', moderate: '#f57c00', mild: '#757575' };
    const severityLabel = { definitive: 'DEFINITIVE', strong: 'STRONG', moderate: 'MODERATE', mild: 'MILD' };

    filteredRows.sort((a, b) => severityOrder[a.severity] - severityOrder[b.severity] || b.count - a.count);

    DOM.breakdownBody.innerHTML = filteredRows.map(row => `
        <tr>
            <td>
                <strong>${row.name}</strong>
                <span style="font-size:10px; color:${severityColor[row.severity]}; margin-left:6px;">${severityLabel[row.severity]}</span>
            </td>
            <td>${row.count}</td>
            <td style="color:#666; font-size:12px;">${row.desc}</td>
        </tr>
    `).join('');
}

function populateFlaggedPhrases(phrases) {
    if (!phrases || phrases.length === 0) {
        DOM.flaggedSection.style.display = 'none';
        return;
    }

    DOM.flaggedSection.style.display = 'block';
    DOM.flaggedList.innerHTML = phrases
        .map(phrase => `<span class="flagged-tag">${escapeHtml(phrase)}</span>`)
        .join('');
}

function clearText() {
    DOM.textInput.value = '';
    DOM.resultsSection.style.display = 'none';
}

function loadSample() {
    DOM.textInput.value = SAMPLE_TEXT;
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
