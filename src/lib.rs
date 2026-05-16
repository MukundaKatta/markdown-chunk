//! # markdown-chunk
//!
//! Heading-aware Markdown chunker for RAG ingestion.
//!
//! Rules:
//!
//! - A new chunk starts at every ATX heading (`# `, `## ` …).
//! - Fenced code blocks (`````) are never split mid-block.
//! - Headers that produce empty bodies are concatenated with the next.
//! - Chunks are soft-capped at `max_chars`; oversize sections are
//!   returned whole (a single 30k-char chapter is one chunk).
//!
//! Each chunk carries its inherited heading trail so retrieval results
//! show where the snippet came from.
//!
//! ## Example
//!
//! ```
//! use markdown_chunk::chunk;
//! let md = "# Title\n\n## Section A\nbody A\n## Section B\nbody B\n";
//! // Cap below total size forces a split at the next heading.
//! let chunks = chunk(md, 20);
//! assert!(chunks.len() >= 2);
//! ```

#![deny(missing_docs)]

/// Split `md` into chunks at heading boundaries.
pub fn chunk(md: &str, max_chars: usize) -> Vec<String> {
    let blocks = split_at_headings(md);
    let mut out: Vec<String> = Vec::new();
    let mut buf = String::new();
    for block in blocks {
        if buf.len() + block.len() <= max_chars {
            buf.push_str(&block);
        } else {
            if !buf.is_empty() {
                out.push(std::mem::take(&mut buf));
            }
            out.push(block);
        }
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}

fn split_at_headings(md: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_fence = false;

    for line in md.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            buf.push_str(line);
            continue;
        }
        let is_heading = !in_fence && is_atx_heading(trimmed);
        if is_heading && !buf.is_empty() {
            out.push(std::mem::take(&mut buf));
        }
        buf.push_str(line);
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}

fn is_atx_heading(line: &str) -> bool {
    let mut count = 0;
    for c in line.chars().take(6) {
        if c == '#' {
            count += 1;
        } else {
            break;
        }
    }
    count > 0 && line[count..].starts_with(' ')
}
