use oxc::{
    allocator::Allocator,
    ast::CommentKind,
    parser::{Parser, ParserReturn},
    span::SourceType,
};
use std::path::Path;

pub struct CommentData {
    pub text: String,
    pub start: usize,
    pub end: usize,
    pub kind: CommentKind,
}

pub fn get_comments(source: &str, source_filename: &str) -> Vec<CommentData> {
    let source_path = Path::new(source_filename);
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(source_path).unwrap();

    let ParserReturn {
        program,
        errors: _parser_errors,
        ..
    } = Parser::new(&allocator, source, source_type).parse();

    let comments: Vec<CommentData> = program
        .comments
        .iter()
        .map(|comment| {
            let start = comment.span.start as usize;
            let end = comment.span.end as usize;
            let text = source[start..end].to_string();
            CommentData {
                text,
                start,
                end,
                kind: comment.kind,
            }
        })
        .collect();

    comments
}
