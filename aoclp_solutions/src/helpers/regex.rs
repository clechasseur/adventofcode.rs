use std::str::FromStr;

use regex::Captures;

pub trait CapturesHelper {
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
