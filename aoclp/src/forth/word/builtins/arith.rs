//! Definition of built-in arithmetic Forth words, like `+`.

use crate::forth;
use crate::forth::Value;
use crate::forth::stack::Stack;
use crate::forth::word::Words;

// A little helper macro to save on code duplication for simple arithmetic operators.
macro_rules! arith_op {
    (
        $(#[$attr:meta])*
        $vis:vis $nam:ident($arith_op:tt);
    ) => {
        $(#[$attr])*
        $vis fn $nam(stack: &mut $crate::forth::stack::Stack, _dictionary: &$crate::forth::word::Words) -> $crate::forth::Result<()> {
            let b = stack.pop()?;
            let a = stack.pop()?;
            stack.push(a $arith_op b);
            Ok(())
        }
    };
}

arith_op! {
    /// Implementation of the `+` Forth word.
    ///
    /// Pops two [`Value`]s from the stack, adds them together and pushes the result to the stack.
    ///
    /// # Errors
    ///
    /// - [`Error::StackUnderflow`] - the stack does not have two values.
    ///
    /// [`Value`]: crate::Value
    pub plus(+);
}
arith_op! {
    /// Implementation of the `-` Forth word.
    ///
    /// Pops two [`Value`]s from the stack, subtract them and pushes the result to the stack. (The
    /// first value popped will be subtracted from the second value popped.)
    ///
    /// # Errors
    ///
    /// - [`Error::StackUnderflow`] - the stack does not have two values.
    ///
    /// [`Value`]: crate::Value
    pub minus(-);
}
arith_op! {
    /// Implementation of the `*` Forth word.
    ///
    /// Pops two [`Value`]s from the stack, multiplies them together and pushes the result to the stack.
    ///
    /// # Errors
    ///
    /// - [`Error::StackUnderflow`] - the stack does not have two values.
    ///
    /// [`Value`]: crate::Value
    pub times(*);
}

/// Implementation of the `/` Forth word.
///
/// Pops the denominator then the numerator from the stack, divides the numerator by the denominator
/// and pushes the result to the stack.
///
/// # Errors
///
/// - [`Error::StackUnderflow`] - the stack does not have two values.
/// - [`Error::DivisionByZero`] - the denominator is 0.
///
/// [`Value`]: crate::Value
pub fn quot(stack: &mut Stack, _dictionary: &Words) -> forth::Result<()> {
    let den = stack.pop()?;
    let num = stack.pop()?;
    match den {
        0 => Err(forth::Error::DivisionByZero),
        den => {
            stack.push(num / den);
            Ok(())
        },
    }
}

// AoC addition:
pub fn concatenate(stack: &mut Stack, _dictionary: &Words) -> forth::Result<()> {
    let right = stack.pop()?;
    let left = stack.pop()?;
    let merged: Value = format!("{left}{right}")
        .parse()
        .map_err(|_| forth::Error::InvalidWord)?;
    stack.push(merged);
    Ok(())
}
