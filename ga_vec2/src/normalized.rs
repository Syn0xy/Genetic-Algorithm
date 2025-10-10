pub trait Normalized {
    type Output;

    fn length_squared(&self) -> f32;
    fn length(&self) -> f32;
    fn normalize_or(&self, default: Self::Output) -> Self::Output;
    fn normalize_or_zero(&self) -> Self::Output;
}
