//! Functional programming helpers.

/// Helper trait providing the [`with_ref`] method for functions that accept
/// a single parameter by value.
///
/// [`with_ref`]: Self::with_ref
pub trait ConsumingPredHelper<T, U> {
    /// Given a function that accepts a single parameter by value, returns a function that receives
    /// the parameter by reference, clones it (or copies it if `T` is `Copy`) then calls the
    /// original function.
    ///
    /// ## Example
    ///
    /// ```
    /// # use aoclp::functional::ConsumingPredHelper;
    ///
    /// let f = |x: String| x.len();
    /// let by_ref = f.with_ref();
    ///
    /// let s = "Hello, world!".to_string();
    /// let s = &s;
    /// let len = by_ref(s);
    /// assert_eq!(len, 13);
    /// ```
    fn with_ref(self) -> impl Fn(&T) -> U
    where
        T: Clone;
}

impl<F, T, U> ConsumingPredHelper<T, U> for F
where
    F: Fn(T) -> U,
{
    fn with_ref(self) -> impl Fn(&T) -> U
    where
        T: Clone,
    {
        move |x| self(x.clone())
    }
}

/// Helper trait providing the [`without_ref`] method for functions that accept
/// a single parameter by reference.
///
/// [`without_ref`]: Self::without_ref
pub trait ByRefPredHelper<T, U> {
    /// Given a function that accepts a single parameter by reference, returns a function that
    /// receives the parameter by value, then calls the original function with the value's reference.
    ///
    /// ## Example
    ///
    /// ```
    /// # use aoclp::functional::ByRefPredHelper;
    ///
    /// let f = |x: &String| x.len();
    /// let by_val = f.without_ref();
    ///
    /// let s = "Hello, world!".to_string();
    /// let len = by_val(s);
    /// assert_eq!(len, 13);
    /// ```
    fn without_ref(self) -> impl Fn(T) -> U;
}

impl<F, T, U> ByRefPredHelper<T, U> for F
where
    F: Fn(&T) -> U,
{
    fn without_ref(self) -> impl Fn(T) -> U {
        move |x| self(&x)
    }
}
