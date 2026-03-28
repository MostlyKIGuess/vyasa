# Agents

## Philosophy

Vyasa won't score!! it will be used to gauge and show patterns, but it won't say "this is 80% likely to be AI-written".

LLM-generated text has identifiable traits: overused vocabulary, formulaic structures, superficial analyses tacked onto sentences, hedging disclaimers, and more. These patterns are well-documented by [Wikipedia's Signs of AI Writing](https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing) and backed by academic research.

Vyasa detects these patterns and shows them to the human. Many patterns appear in human writing too. The human decides what the patterns mean.

All analysis runs in the browser via WebAssembly. Text never leaves the machine.

## For AI agents working on this codebase

Read `.rules` before writing code. Key points:

- No comments that summarize code. Only explain *why* something is non-obvious.
- No `unwrap()`. Use `?` or `expect()` with a reason.
- Full variable names. No abbreviations.
- Prefer editing existing files over creating new ones.
- No creative additions unless asked.

Detection patterns live in `src/indicators.rs`. Scoring weights live in `src/scoring.rs`. Data structures in `src/models.rs`. The WASM entry point is in `src/lib.rs`. The server (optional, for local dev) is in `src/main.rs`.

When adding a new indicator:
1. Add the field to `IndicatorBreakdown` in `models.rs`
2. Add the check function and wordlist/regex in `indicators.rs`
3. Wire it into `analyze_indicators()` in `indicators.rs`
4. Add a weight in `calculate_pattern_score()` in `scoring.rs`
5. Add a row in the breakdown table in `static/app.js`
6. Add a test

## API

The server exposes `POST /api/detect` accepting `{"text": "...", "max_chars": 1000000}` and returning the full analysis as JSON. The WASM module exposes `analyze_text(text, max_chars)` returning the same JSON as a string.
