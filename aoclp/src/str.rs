use std::str::FromStr;

pub trait StrHelper {
    /// Splits the string at the given position and parses each of the
    /// two segments using [`parse`].
    ///
    /// Panics if `pos` is not a valid index in the string.
    ///
    /// ## Note
    ///
    /// The character at `pos` will be part of the second segment.
    ///
    /// ## Example
    ///
    /// ```
    /// # use std::str::FromStr;
    ///
    /// # use anyhow::anyhow;
    /// # use aoclp::str::StrHelper;
    ///
    /// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    /// enum RotationDirection {
    ///     Left,
    ///     Right,
    /// }
    ///
    /// impl FromStr for RotationDirection {
    ///     type Err = aoclp::Error;
    ///
    ///     fn from_str(s: &str) -> Result<Self, Self::Err> {
    ///         match s {
    ///             "LEFT" => Ok(Self::Left),
    ///             "RGHT" => Ok(Self::Right),
    ///             s => Err(anyhow!("must be LEFT or RGHT, got {s}")),
    ///         }
    ///     }
    /// }
    ///
    /// let data = "RGHT2342";
    /// let (dir, len) = data.split_parse_at(4);
    /// assert_eq!(RotationDirection::Right, dir);
    /// assert_eq!(2342, len);
    /// ```
    ///
    /// [`parse`]: str::parse
    fn split_parse_at<T, U>(&self, pos: usize) -> (T, U)
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug;
}

impl<S> StrHelper for S
where
    S: AsRef<str>,
{
    fn split_parse_at<T, U>(&self, pos: usize) -> (T, U)
    where
        T: FromStr,
        U: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
        <U as FromStr>::Err: std::fmt::Debug,
    {
        let s = self.as_ref();
        (s[0..pos].parse().unwrap(), s[pos..].parse().unwrap())
    }
}
