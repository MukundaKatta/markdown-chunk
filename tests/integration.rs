use markdown_chunk::chunk;

#[test]
fn splits_on_headings() {
    let md = "# Title\n\n## A\nbody A\n## B\nbody B\n";
    // Cap below the merged size so heading boundaries actually split.
    let chunks = chunk(md, 15);
    assert!(chunks.len() >= 2, "got {chunks:?}");
}

#[test]
fn does_not_split_inside_fence() {
    let md = "## A\n```\n# fake heading\nmore code\n```\n## B\n";
    let chunks = chunk(md, 1000);
    // The fake heading inside the fence must not start a new chunk.
    let count_a = chunks
        .iter()
        .filter(|c| c.contains("## A"))
        .count();
    assert_eq!(count_a, 1);
}

#[test]
fn merges_small_sections_under_cap() {
    let md = "# T\n\n## A\nx\n## B\ny\n";
    let huge_cap = 10_000;
    let chunks = chunk(md, huge_cap);
    assert_eq!(chunks.len(), 1);
}

#[test]
fn oversize_section_returned_whole() {
    let body = "lorem ipsum ".repeat(200);
    let md = format!("## Big\n{body}");
    let chunks = chunk(&md, 50);
    assert_eq!(chunks.len(), 1);
    assert!(chunks[0].contains("## Big"));
}

#[test]
fn empty_input() {
    let chunks = chunk("", 100);
    assert!(chunks.is_empty());
}
