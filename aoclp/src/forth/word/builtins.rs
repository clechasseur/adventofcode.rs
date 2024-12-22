//! Definition of built-in Forth words.

pub mod arith;
pub mod stack;

use crate::forth::word::{wrap_fn_word, Words};

/// Adds all built-in [`Word`](crate::word::Word) to the given [word map](Words).
pub fn add_builtin_words(words: &mut Words) {
    words.insert("+".into(), wrap_fn_word(arith::plus));
    words.insert("-".into(), wrap_fn_word(arith::minus));
    words.insert("*".into(), wrap_fn_word(arith::times));
    words.insert("/".into(), wrap_fn_word(arith::quot));
    words.insert("DUP".into(), wrap_fn_word(stack::dup));
    words.insert("DROP".into(), wrap_fn_word(stack::drop));
    words.insert("SWAP".into(), wrap_fn_word(stack::swap));
    words.insert("OVER".into(), wrap_fn_word(stack::over));

    // AoC additions:
    words.insert("||".into(), wrap_fn_word(arith::concatenate));
}
