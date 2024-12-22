//! Types used to describe words understood by the [`Forth`](crate::Forth) interpreter.

pub mod builtins;
pub mod custom;
pub mod value;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use crate::forth;
use crate::forth::stack::Stack;

/// Implementation of a Forth word.
///
/// Acts a bit like a [`Fn`]: can be [`call`]ed to interact with the [`Forth`] interpreter's [`Stack`].
///
/// [`Forth`]: crate::Forth
/// [`call`]: Word::call
pub trait Word {
    /// "Calls" the word, instructing it to perform its operation on the Forth interpreter's stack.
    ///
    /// The word should use the stack to fetch any parameter required and any output from the operation
    /// should then be pushed to the stack.
    ///
    /// The provided dictionary can be used to fetch definitions for other words; it will be used
    /// to implement [custom words](custom::CustomWord).
    fn call(&self, stack: &mut Stack, dictionary: &Words) -> forth::Result<()>;
}

/// A Forth [`Word`], stored in a [`Rc`]. Used to store dynamic words in structures.
///
/// We use [`Rc`] so that when we store references to existing words in custom ones,
/// we do not duplicate them.
pub type WordRc = Rc<dyn Word>;

/// A dictionary of known [`Word`]s.
///
/// Each word is stored in a [`WordRc`] and mapped to its string representation.
///
/// # Notes
///
/// Words' string representations are compared case-sensitively in the dictionary.
/// In order to use it properly for Forth, make sure to convert words to uppercase.
#[derive(Default, Clone)]
pub struct Words {
    data: HashMap<String, WordRc>,
}

impl Words {
    /// Creates an empty dictionary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a new [`Word`] in the dictionary.
    ///
    /// If an existing [`Word`] was already associated with this word in the dictionary,
    /// it will be returned.
    pub fn insert(&mut self, word: String, op: WordRc) -> Option<WordRc> {
        self.data.insert(word, op)
    }

    /// Looks for a [`Word`] in the dictionary.
    ///
    /// # Errors
    ///
    /// - [`Error::UnknownWord`] - Word is not in the dictionary.
    pub fn get<Q>(&self, word: &Q) -> forth::Result<WordRc>
    where
        String: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.data
            .get(word)
            .cloned()
            .ok_or(forth::Error::UnknownWord)
    }
}

impl<F> Word for F
where
    F: Fn(&mut Stack, &Words) -> forth::Result<()>,
{
    /// Implementation of [`Word`] for a function with the same signature as [`Word::call`].
    ///
    /// Will be used to define built-in words like `+`, `DUP`, etc.
    ///
    /// To create, use [`wrap_fn_word`].
    fn call(&self, stack: &mut Stack, dictionary: &Words) -> forth::Result<()> {
        self(stack, dictionary)
    }
}

/// Given a function that has a signature similar to [`Word::call`], wraps it into a [`WordRc`].
///
/// # Examples
///
/// ```
/// use std::ops::Deref;
///
/// use aoclp::forth;
/// use aoclp::forth::stack::Stack;
/// use aoclp::forth::word::{wrap_fn_word, Words};
///
/// fn add_10(stack: &mut Stack, _dictionary: &Words) -> forth::Result<()> {
///     let value = stack.pop()?;
///     stack.push(value + 10);
///     Ok(())
/// }
/// let add_10_word = wrap_fn_word(add_10);
///
/// let mut stack = Stack::new();
/// stack.push(2);
/// let dictionary = Words::new();
///
/// assert!(add_10_word.call(&mut stack, &dictionary).is_ok());
/// assert_eq!(&[12], stack.deref());
/// ```
pub fn wrap_fn_word<F>(f: F) -> WordRc
where
    F: Fn(&mut Stack, &Words) -> forth::Result<()> + 'static,
{
    Rc::new(f)
}
