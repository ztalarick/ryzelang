# Ryzelang Language Specification

## 1. Overview
Ryzelang is an esoteric, Turing-complete, functional stack-based programming language modeled after the gameplay mechanics of the League of Legends champion, Ryze. 

Programs are constructed by chaining "Spells" (primitives) and "Combos" (macros/functions). The language uses a dual-stack architecture to separate data (Runes) from behavior (Scrolls).

## 2. Core Mechanics

### The Rune Stack (Data Stack)
The primary data structure for calculation. It is a Last-In, First-Out (LIFO) stack that holds arbitrary precision integers.

### The Scroll Stack (Control Stack)
A separate LIFO stack that holds "Blocks" (unexecuted sequences of code). This allows code to be treated as data for conditionals and definitions.

### The Scroll (Environment)
The global dictionary that maps Combo Names (e.g., `QWE`) to specific Blocks.

### Execution Model: The "Wait-and-See" (Lookahead)
The interpreter reads tokens separated by whitespace. To distinguish between calling a combo and naming one for a definition, it uses **LL(1) Lookahead**:
- When it encounters a `<combo_token>` (e.g., `QQE`), it buffers it and looks at the **next** token.
- If the next token is `R`, the buffered token is treated as a **Name** for the `R` (Memorize) operation.
- Otherwise, the buffered token is treated as a **Call** and executed immediately.

## 3. Syntax

### Spells (Tokens)
The only valid alphabetic characters are `Q`, `W`, `E`, and `R`.
A Combo is any contiguous string of `Q`, `W`, and `E`. Spaces separate Combos.

### Blocks `[ ... ]`
Enclosing code in brackets creates a "Block". When the interpreter encounters a block, it is **not** executed; instead, it is pushed onto the **Scroll Stack**.

### Memorization (R)
The `R` (Realm Warp) command binds a block to a name.
**Format:** `[ <body> ] <name> R`
1. Pops the top block from the **Scroll Stack**.
2. Binds it to the `<name>` token in the Environment.

## 4. Standard Library (Built-in Combos)

### Rune Stack Operations (Numbers)
- `Q` : Push `1` onto the **Rune Stack**.
- `W` : Pop the top value from the **Rune Stack** and discard it.
- `E` : Duplicate the top value of the **Rune Stack**.
- `EW` : Swap the top two values of the **Rune Stack**.

### Mathematics (Rune Stack)
- `QQ` : Pop top two, add them, push result.
- `QW` : Pop top two, subtract top from second-top, push result.
- `WE` : Pop top two, multiply them, push result.
- `WQ` : Pop top two, divide second-top by top, push result.

### Logic & Control Flow (Dual Stack)
- `EQ` : Overload / Is Zero. 
  - If the top Rune is a **normal integer**: Pops it. If `0`, pushes `1`. Otherwise, pushes `0`.
  - If the top Rune is **Flux-Marked**: "Unwraps" it. Pushes the value and a `1` if it was "Charged", or just a `0` if it was "Empty".
- `WEQ` : Conditional execution. 
  1. Pops **one integer** from the **Rune Stack** (the condition).
  2. Pops **two blocks** from the **Scroll Stack** (True and False paths).
  3. If condition `> 0`, executes the first block. Otherwise, executes the second.
- `WWW` : **Rune Prison**. Pops the top of the **Rune Stack** and pauses execution for that many milliseconds.
- `EE` : **Spell Flux**. Consumes the top Rune. If the value is `0`, it pushes a "Flux-Empty" state. If `!= 0`, it pushes a "Flux-Charged" state containing that value.

### Input / Output
- `EWQ` : Pop top Rune value and print as an integer.
- `EQE` : Pop top Rune value and print as an ASCII character.
- `EEW` : Read one ASCII character and push its integer value to the **Rune Stack**.

## 5. Examples

### Defining a "Double" Combo
`[ E QQ ] QQE R`
- `[ E QQ ]` is pushed to Scroll Stack.
- `QQE` is buffered.
- `R` sees the buffer, pops the block, and stores it.

### Using the Combo
`Q Q Q QQE EWQ`
- Pushes 1, 1, 1 (Stack: `[1, 1, 1]`).
- `QQE` is called (Stack: `[1, 1, 2]`).
- `EWQ` prints `2`.
