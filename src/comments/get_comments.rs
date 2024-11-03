use oxc::{
    allocator::Allocator,
    ast::CommentKind,
    parser::{Parser, ParserReturn},
    span::{SourceType, Span},
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
    let mut errors = Vec::new();

    let ParserReturn {
        trivias,
        errors: parser_errors,
        ..
    } = Parser::new(&allocator, source, source_type).parse();

    errors.extend(parser_errors);

    let comments: Vec<CommentData> = trivias
        .comments()
        .map(|comment| {
            let Span { start, end, .. } = comment.span;
            let text = source[start as usize..end as usize].to_string();
            CommentData {
                text,
                start: start as usize,
                end: end as usize,
                kind: comment.kind,
            }
        })
        .collect();

    comments
}
