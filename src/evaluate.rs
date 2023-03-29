pub mod value_map;

pub trait Evaluate {
    type X;
    type Y;
    fn evaluate(&self, x: Self::X) -> Self::Y;
}
