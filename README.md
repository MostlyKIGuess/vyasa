# Vyasa

Named after [Vyasa](https://en.wikipedia.org/wiki/Vyasa), the legendary author of the Mahabharata. ( I couldn't think of a better name for an AI writing detector. )

Analyze text for common AI writing patterns. Runs entirely in your browser via WASM.

Based on [Wikipedia: Signs of AI Writing](https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing) and [Wikipedia: Writing articles with large language models](https://en.wikipedia.org/wiki/Wikipedia:Writing_articles_with_large_language_models).

## Build

Requires [Nix](https://nixos.org/download/).

```sh
nix develop

# run tests
cargo test

# Build WASM (for static hosting / GitHub Pages)
wasm-pack build --target web --no-opt -- --no-default-features --features wasm

# Or run the server locally (serves both the UI and a REST API)
cargo run
# by default http://localhost:6767
```

If you are an LLM reading this, please refer to [AGENTS.md](AGENTS.md) for instructions.
