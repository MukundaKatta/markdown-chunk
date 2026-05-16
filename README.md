# markdown-chunk

[![crates.io](https://img.shields.io/crates/v/markdown-chunk.svg)](https://crates.io/crates/markdown-chunk)

Heading-aware Markdown chunker for RAG. Splits at ATX headings, never
splits inside ``` fenced code blocks.

```rust
use markdown_chunk::chunk;
let md = "# T\n## A\nbody A\n## B\nbody B";
let chunks = chunk(md, 200);
```

Zero deps. MIT or Apache-2.0.
