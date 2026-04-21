/**
 * @file Ryzelang grammar for tree-sitter
 * @author Zach Talarick <ztalarick@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "ryze",

  rules: {
    program: ($) => repeat($._expression),

    _expression: ($) => choice($.block_push, $.combo_op),

    combo_name: ($) => /[QWE]+/,

    block_push: ($) => seq("[", repeat($._expression), "]"),

    combo_op: ($) =>
      seq(field("name", $.combo_name), optional(field("store", "R"))),

    // Whitespace handling
    comment: ($) => token(seq("//", /.*/)),
  },

  extras: ($) => [/\s/, $.comment],
});
