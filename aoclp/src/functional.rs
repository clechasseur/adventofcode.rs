pub trait PredHelper<T, U> {
    fn with_ref(self) -> impl Fn(&T) -> U
    where
        T: Clone;
}

impl<F, T, U> PredHelper<T, U> for F
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
