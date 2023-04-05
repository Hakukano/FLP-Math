pub mod expression;
pub mod value_map;

/// Evaluate from x to y
///
/// # Generic
///
/// - `X` - from type
/// - `Y` - to type
pub trait Evaluate<X, Y> {
    fn evaluate(&self, x: X) -> Y;
}
