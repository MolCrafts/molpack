//! Module for parsing various chemfile formats like `xyz`.
pub mod xyz;

use winnow::error::{ContextError, ParseError};

/// Common error type for parsing chemfiles
#[derive(Debug)]
pub struct ParseChemfileError {
    message: String,

    span: std::ops::Range<usize>,
    input: String,
}

impl ParseChemfileError {
    fn from_parse(error: ParseError<&str, ContextError>, input: &str) -> Self {
        let message = error.inner().to_string();
        let input = input.to_owned();
        let start = error.offset();

        // Assume the error span is only fot the first `char`
        let end = (start + 1..)
            .find(|e| input.is_char_boundary(*e))
            .unwrap_or(start);
        Self {
            message,
            span: start..end,
            input,
        }
    }
}

impl std::fmt::Display for ParseChemfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = annotate_snippets::Level::Error
            .title(&self.message)
            .snippet(
                annotate_snippets::Snippet::source(&self.input)
                    .fold(true)
                    .annotation(annotate_snippets::Level::Error.span(self.span.clone())),
            );
        let renderer = annotate_snippets::Renderer::plain();
        let rendered = renderer.render(message);
        rendered.fmt(f)
    }
}

impl std::error::Error for ParseChemfileError {}
