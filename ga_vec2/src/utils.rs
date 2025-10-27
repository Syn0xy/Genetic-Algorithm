use rand::distr;

pub trait Normalized {
    type Output;

    fn length_squared(&self) -> f32;
    fn normalize_or(&self, default: Self::Output) -> Self::Output;
    fn normalize_or_zero(&self) -> Self::Output;

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

pub trait Randomized<T>
where
    T: distr::uniform::SampleUniform,
{
    type Output;

    fn random_range<R>(range: R) -> Self::Output
    where
        R: distr::uniform::SampleRange<T> + Clone;
}
