use std::{collections::HashMap, hash::Hash};

/// Evaluate with x-y map.
pub struct Evaluate<X, Y>(pub HashMap<X, Y>);

impl<X, Y> super::Evaluate<X, Y> for Evaluate<X, Y>
where
    X: PartialEq + Eq + Hash,
    Y: Clone + Copy,
{
    fn evaluate(&self, x: X) -> Y {
        *self.0.get(&x).expect("Out of bound: ValueMap")
    }
}
