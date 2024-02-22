````rust
#[derive(Default)]
pub struct Tuple<F, S>(pub F, pub S);

trait TupleMark {}
pub trait Concat<T> {
    type Output;
    fn join(self, v: T) -> Self::Output;
}
impl<F, S, T> Concat<T> for Tuple<F, S>
where
    S: Concat<T>,
    T: TupleMark,
{
    type Output = Tuple<F, S::Output>;
    fn join(self, v: T) -> Self::Output {
        Tuple(self.0, self.1.join(v))
    }
}
impl<T> Concat<T> for ()
where
    T: TupleMark,
{
    type Output = T;
    fn join(self, v: T) -> Self::Output {
        v
    }
}
impl<F, S: TupleMark> TupleMark for Tuple<F, S> {}
impl TupleMark for () {}
````
