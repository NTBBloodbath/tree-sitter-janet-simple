use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_janet_simple() -> *const ();
}

pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_janet_simple) };

pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&crate::LANGUAGE.into())
            .expect("Error loading janet_simple language");
    }

    #[test]
    fn test_highlights_query_compiles() {
        let language = crate::LANGUAGE.into();
        tree_sitter::Query::new(&language, crate::HIGHLIGHTS_QUERY)
            .expect("highlights query must compile against the grammar");
    }
}
