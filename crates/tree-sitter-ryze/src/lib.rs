use tree_sitter::Language;

extern "C" {
    fn tree_sitter_ryze() -> Language;
}

/// Get the tree-sitter [Language] for this grammar.
pub fn language() -> Language {
    unsafe { tree_sitter_ryze() }
}

/// The content of the [`node-types.json`] file for this grammar.
pub const NODE_TYPES: &str = include_str!("../../../tree-sitter-ryze/src/node-types.json");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language())
            .expect("Error loading Ryzelang grammar");
    }
}
