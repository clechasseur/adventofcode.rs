//! Defines a stack wrapper used by the [`Forth`](crate::Forth) interpreter.

use std::ops::Deref;

use crate::helpers::forth;
use crate::helpers::forth::Value;

/// A simple stack wrapper over [`Vec`].
///
/// Implements the methods required by the Forth interpreter. Methods that can fail
/// will return a [`Result`] so that the interpreter can easily return early when using them.
#[derive(Debug, Default, Clone)]
pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    /// Creates a new, empty [`Stack`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the [`Value`] at the top of the stack without removing it.
    ///
    /// # Errors
    ///
    /// - [`Error::StackUnderflow`] - the stack is empty.
    pub fn peek(&self) -> forth::Result<Value> {
        self.values
            .last()
            .copied()
            .ok_or(forth::Error::StackUnderflow)
    }

    /// Pushes a [`Value`] on the top of the stack.
    pub fn push(&mut self, v: Value) {
        self.values.push(v);
    }

    /// Removes the [`Value`] at the top of the stack and returns it.
    ///
    /// # Errors
    ///
    /// [`Error::StackUnderflow`] - the stack is empty.
    pub fn pop(&mut self) -> forth::Result<Value> {
        self.values.pop().ok_or(forth::Error::StackUnderflow)
    }
}

impl Deref for Stack {
    type Target = [Value];

    /// Returns a reference to the [`Value`]s on the stack, from bottom to top.
    ///
    /// Provided so that it's possible to use `&stack` to access the stack's values,
    /// like you would with a [`Vec`].
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}
