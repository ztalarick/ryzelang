# Language Server Protocol (LSP)

The Ryzelang Language Server (`ryzelang-ls`) provides an intelligent editing experience by implementing the Language Server Protocol. It is written in Rust using the `tower-lsp` framework and the `tree-sitter-ryze` grammar.

## Editor Integration

The primary client is the **VS Code Extension** located in `vscode-ryzelang/`. To use the language server in VS Code:
1. Build the server binary: `cargo build --package ryzelang-ls`.
2. Configure the extension to point to the built binary (see `vscode-ryzelang/README.md`).

## Features

### 1. Diagnostics
The server continuously parses the document using Tree-sitter. If it encounters `ERROR` or `MISSING` nodes in the AST, it reports them as **Syntax Errors** in your editor's "Problems" tab.

### 2. Hover Documentation
Hovering over any built-in **Spell** (e.g., `QQ`, `WEQ`) displays its documentation, including:
- A brief description of its function.
- Its effect on the **Rune Stack** and **Scroll Stack**.

### 3. Completion
The server provides autocompletion suggestions for all 16 built-in primitives whenever you start typing a combo.

### 4. Go-to-Definition
For user-defined **Combos**, you can use "Go to Definition" (F12) on a combo call to warp to the location where it was bound using the `R` operator.

## Architecture

The server maintains a `document_map` using `DashMap` for thread-safe access to open files. 

1. **`on_change`**: Every time the file is edited, the server re-parses the source and publishes new diagnostics.
2. **Tree-sitter Integration**: The server uses the formal Ryzelang grammar to navigate the code structure, allowing it to precisely identify combo names and definitions.

## Manual Usage

While typically launched by an editor extension, the server can be run manually for debugging:

```bash
cargo run --package ryzelang-ls
```

It communicates over `stdin`/`stdout` using JSON-RPC.
