# Ryzelang Formal Grammar

This document defines the formal grammar for Ryzelang using Extended Backus-Naur Form (EBNF).

## Lexical Grammar

Whitespace is used strictly as a separator between tokens.

```ebnf
<whitespace>  ::= " " | "\t" | "\n" | "\r"
<qwe_char>    ::= "Q" | "W" | "E"
<r_char>      ::= "R"
<l_bracket>   ::= "["
<r_bracket>   ::= "]"

<combo_token> ::= <qwe_char>+
<store_token> ::= <r_char>
```

## Syntactic Grammar

A Ryzelang program is a sequence of expressions evaluated from left to right with 1-token lookahead.

```ebnf
<program>     ::= <expression>*

<expression>  ::= <store_op>
                | <combo_call> 
                | <block_push>

<block_push>  ::= <l_bracket> <whitespace>* <program> <whitespace>* <r_bracket>
                  (* Pushes a Block to the Scroll Stack *)

<store_op>    ::= <combo_token> <whitespace>+ <store_token>
                  (* Consumes top of Scroll Stack and binds to <combo_token> *)

<combo_call>  ::= <combo_token>
                  (* Executes a built-in or custom combo *)
```

### Parsing Logic (Ambiguity Resolution)
Because `<store_op>` and `<combo_call>` both begin with a `<combo_token>`, the parser must look ahead one token:

1. If current token is `<l_bracket>`, parse `<block_push>`.
2. If current token is `<combo_token>`:
   - Peek at next token.
   - If next token is `R`, parse as the name part of `<store_op>`.
   - Otherwise, parse as `<combo_call>`.
3. If current token is `R`, it is a standalone operator (Error if not preceded by a name).

### Runtime State
- **Rune Stack**: `Stack<Integer>`
- **Scroll Stack**: `Stack<Block>`
- **The Scroll**: `Map<String, Block>`
