extern crate pulldown_cmark;

/// Parse only a subset of Common Markdown into an intermediate AST object.
mod ast;

use std::io::{self, Read};

fn read_stdin() -> Result<String, String> {
    let mut s = String::new();
    io::stdin()
        .read_to_string(&mut s)
        .map_err(|e| e.to_string())?;
    Ok(s)
}

fn main() -> Result<(), String> {
    let text = read_stdin()?;
    // Test print token stream
    for event in pulldown_cmark::Parser::new(&text) {
        println!("{:?}", event)
    }
    // Ast test
    let (root, keywords) = ast::parse(&text)?;
    println!("AST: {:?}", root);
    println!("KWDS: {:?}", keywords);
    Ok(())
}

// Modes:
// - wiki : generates a wiki
// - keyword : extract list of keywords

// Description / Associate many things with keywords:
// Sentence version "<kwd> : text ; text ; text."
// List version "<kwd>:\n- <text>\n- <text>"

// Keyword occurence -> description "kwd: <text>", and just occurence.

// Wiki : group data by keywords
// - show data in order of .md file (headings, etc). links for keywords
// - by keyword, list of sentences organised by heading position

/*struct OriginPosition {
    lines: ops::Range<usize>,
    file: usize,
}
struct Sentence {
    sentence: String,
    heading: HeadingPosition,
    origin: OriginPosition,
}
struct Heading {
    text: String,
    sub_headings: Vec<Heading>,
    sentences: ops::Range<usize>,
    origin: OriginPosition,
}
struct HeadingPosition {
    file_index: usize,
    heading_indexes: [Option<usize>; 6],
}
struct File {
    filename: String,
    headings: Vec<Heading>,
}
struct Keyword {
    keyword: String,
    in_sentences: Vec<usize>,
}

struct Database {
    /// All sentences (not heading)
    sentences: Vec<Sentence>,
    /// File & heading tree
    files: Vec<File>,
    /// All found keywords
    keywords: Vec<Keyword>,
    keyword_indexes: HashMap<String, usize>,
}*/
