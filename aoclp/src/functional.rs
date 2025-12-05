pub trait ConsumingPredHelper<T, U> {
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

pub trait ByRefPredHelper<T, U> {
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
