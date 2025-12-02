use std::str::FromStr;

pub trait StrHelper {
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
