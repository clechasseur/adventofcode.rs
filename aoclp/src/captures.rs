use std::str::FromStr;

use crate::regex::Captures;

/// Helpers for regex [`Captures`].
pub trait CapturesHelper {
    /// Returns the content of the given named capture group, converted
    /// to a value implementing `FromStr`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use std::convert::Infallible;
    /// # use std::str::FromStr;
    /// # use std::sync::OnceLock;
    ///
    /// # use regex::Regex;
    /// # use aoclp::captures::CapturesHelper;
    /// # use aoclp::positioning::pt_3d::Pt3d;
    ///
    /// type Coords = Pt3d;
    ///
    /// #[derive(Debug, Default, Copy, Clone)]
    /// struct Particle {
    ///     pub id: usize,
    ///     pub position: Coords,
    ///     pub velocity: Coords,
    ///     pub acceleration: Coords,
    /// }
    ///
    /// impl FromStr for Particle {
    ///     type Err = Infallible;
    ///
    ///     fn from_str(s: &str) -> Result<Self, Self::Err> {
    ///         static REGEX: OnceLock<Regex> = OnceLock::new();
    ///         let re = REGEX.get_or_init(|| {
    ///             Regex::new(r"^p=<(?<p>.+)>,\s*v=<(?<v>.+)>,\s*a=<(?<a>.+)>$").unwrap()
    ///         });
    ///
    ///         let captures = re
    ///             .captures(s)
    ///             .unwrap_or_else(|| panic!("invalid Particle value: {s}"));
    ///         Ok(Self {
    ///             position: captures.ez_get("p"),
    ///             velocity: captures.ez_get("v"),
    ///             acceleration: captures.ez_get("a"),
    ///             ..Self::default()
    ///         })
    ///     }
    /// }
    /// ```
    fn ez_get<T>(&self, name: &str) -> T
    where
        T: FromStr;
}

impl CapturesHelper for Captures<'_> {
    fn ez_get<T>(&self, name: &str) -> T
    where
        T: FromStr,
    {
        self[name]
            .parse::<T>()
            .unwrap_or_else(|_| panic!("invalid value for {name}: {}", &self[name]))
    }
}
