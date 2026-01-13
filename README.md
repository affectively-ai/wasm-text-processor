# @affectively/wasm-text-processor

High-performance WebAssembly text processing utilities written in Rust.

[![npm version](https://img.shields.io/npm/v/@affectively/wasm-text-processor.svg)](https://www.npmjs.com/package/@affectively/wasm-text-processor)
[![crates.io](https://img.shields.io/crates/v/affectively-text-processor.svg)](https://crates.io/crates/affectively-text-processor)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Pattern Matching** - Fast regex and string pattern matching
- **Entity Extraction** - Extract named entities from text
- **Text Scoring** - Score text based on configurable criteria
- **Zero-copy Processing** - Efficient memory usage

## Installation

```bash
npm install @affectively/wasm-text-processor
```

## Quick Start

```typescript
import init, { match_patterns, extract_entities, score_text } from '@affectively/wasm-text-processor';

await init();

// Pattern matching
const matches = match_patterns(text, patterns);

// Entity extraction
const entities = extract_entities(text);

// Text scoring
const score = score_text(text, criteria);
```

## License

MIT License - see [LICENSE](./LICENSE) for details.

---

Made with ️ by [AFFECTIVELY](https://affectively.ai)
