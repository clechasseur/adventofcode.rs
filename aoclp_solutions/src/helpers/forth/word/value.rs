//! [`Word`] implementation for simple values (e.g. numbers).

use std::rc::Rc;

use crate::helpers::forth;
use crate::helpers::forth::stack::Stack;
use crate::helpers::forth::word::{Word, WordRc, Words};
use crate::helpers::forth::Value;

/// A Forth [`Word`] that pushes a [`Value`] on the [`Stack`] when [`call`]ed.
///
/// [`call`]: ValueWord::call
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ValueWord(pub Value);

impl ValueWord {
    /// Creates a [`ValueWord`] and wraps it in a [`WordRc`].
    pub fn wrap(value: Value) -> WordRc {
        Rc::new(ValueWord(value))
    }
}

impl Word for ValueWord {
    /// "Calls" the word, pushing its [`Value`] to the [`Stack`].
    fn call(&self, stack: &mut Stack, _dictionary: &Words) -> forth::Result<()> {
        stack.push(self.0);
        Ok(())
    }
}
