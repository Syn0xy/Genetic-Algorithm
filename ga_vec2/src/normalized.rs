pub trait Normalized {
    type Output;

    fn magnetude(&self) -> f32;
    fn normalize_or(&self, default: Self::Output) -> Self::Output;
    fn normalize_or_zero(&self) -> Self::Output;
}
