fn main() {
    let src_dir = std::path::Path::new("../../tree-sitter-ryze/src");

    let mut config = cc::Build::new();
    config.include(src_dir);
    config.file(src_dir.join("parser.c"));
    
    // If you have a scanner.c, include it too. 
    // Ryzelang is simple enough that it shouldn't have one yet.
    
    config.compile("tree-sitter-ryze");
}
