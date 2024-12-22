//! Definition of custom (e.g. user-defined) [`Word`]s.

use std::rc::Rc;

use crate::helpers::forth;
use crate::helpers::forth::stack::Stack;
use crate::helpers::forth::word::{Word, WordRc, Words};

/// A custom (e.g. user-defined) [`Word`].
///
/// A custom word is an alias for a sequence of [`Word`]s that are executed when the custom word is [`call`]ed.
///
/// [`call`]: CustomWord::call
pub struct CustomWord {
    inner_words: Vec<WordRc>,
}

impl CustomWord {
    /// Creates a new [`CustomWord`] as an alias for a sequence of existing [`Word`]s, then wraps it in a [`WordRc`].
    pub fn wrap(inner_words: Vec<WordRc>) -> WordRc {
        Rc::new(Self { inner_words })
    }
}

impl Word for CustomWord {
    /// "Calls" the custom word, executing its inner sequence of words.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ops::Deref;
    ///
    /// use aoclp_solutions::helpers::forth;
    /// use aoclp_solutions::helpers::forth::stack::Stack;
    /// use aoclp_solutions::helpers::forth::word::builtins::add_builtin_words;
    /// use aoclp_solutions::helpers::forth::word::custom::CustomWord;
    /// use aoclp_solutions::helpers::forth::word::value::ValueWord;
    /// use aoclp_solutions::helpers::forth::word::Words;
    ///
    /// let mut words = Words::new();
    /// add_builtin_words(&mut words);
    ///
    /// let inner_words =
    ///     vec![ValueWord::wrap(1), ValueWord::wrap(2), words.get("+")?, words.get("DUP")?];
    /// let custom_word = CustomWord::wrap(inner_words);
    ///
    /// let mut stack = Stack::new();
    /// assert!(custom_word.call(&mut stack, &words).is_ok());
    /// assert_eq!(&[3, 3], stack.deref());
    /// # Ok::<(), forth::Error>(())
    /// ```
    fn call(&self, stack: &mut Stack, dictionary: &Words) -> forth::Result<()> {
        self.inner_words
            .iter()
            .try_fold((), |_, word| word.call(stack, dictionary))
    }
}
