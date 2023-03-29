use std::{collections::HashMap, hash::Hash};

pub struct Evaluate<X, Y>(pub HashMap<X, Y>);
impl<X, Y> super::Evaluate for Evaluate<X, Y>
where
    X: PartialEq + Eq + Hash,
    Y: Clone + Copy,
{
    type X = X;
    type Y = Y;
    fn evaluate(&self, x: Self::X) -> Self::Y {
        *self.0.get(&x).expect("Out of bound: ValueMap")
    }
}
