# Architecture Overview

Ryzelang is built on a custom dual-stack virtual machine. This document explains the core components and the execution lifecycle.

## The Dual-Stack Model

Unlike traditional stack-based languages (like Forth or PostScript) that use a single stack for both data and control, Ryzelang separates these concerns into two distinct structures:

### 1. The Rune Stack (Data)
- **Content:** Arbitrary-precision integers (`BigInt`).
- **Purpose:** All mathematical calculations, logical comparisons, and I/O values.
- **Flux State:** Runes can be "Marked with Flux" (using the `EE` spell). This creates a specialized state (similar to `Option` or `Result` in other languages) that must be unwrapped using `EQ` before use in math.

### 2. The Scroll Stack (Control)
- **Content:** Code blocks (strings of Ryzelang source).
- **Purpose:** Deferred execution. Blocks are pushed to this stack when encountered in brackets `[...]`.
- **Consumption:** The `WEQ` spell (Conditional) and the `R` operator (Memorization) consume blocks from this stack.

## The Environment (The Scroll)

The "Scroll" is a global hash map that stores user-defined **Combos**.
- **Key:** A string consisting only of `Q`, `W`, and `E`.
- **Value:** A code block string.
- **Protection:** Built-in **Spells** (like `QQ`, `WWW`, `R`) are reserved and cannot be overwritten in the Scroll.

## Execution Lifecycle

1. **Parsing:** The source code is parsed into an Abstract Syntax Tree (AST) using Tree-sitter.
2. **Traversal:** The Interpreter walks the AST.
3. **Block Handling:** If a `block_push` node is encountered, its literal text is pushed to the **Scroll Stack**.
4. **Combo Handling:**
   - If the node is a `combo_op` with an `R` field, the top block is popped from the Scroll Stack and stored in the Environment.
   - Otherwise, the interpreter looks for a built-in **Spell**. If found, it executes the Rust implementation.
   - If no built-in spell exists, it looks for a user-defined **Combo** in the Environment and executes it recursively.
5. **Lookahead:** The language uses LL(1) lookahead to distinguish between calling a combo and defining one.

## Recursion and Scoping

When a combo or block is executed, a sub-interpreter is created. This sub-interpreter shares the same `Runtime` (Stacks and Environment) as the parent, allowing for seamless state persistence across calls.
