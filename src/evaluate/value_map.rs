use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

/// Evaluate with x-y map.
#[derive(Deserialize, Serialize)]
pub struct Evaluate<X, Y>(pub HashMap<X, Y>)
where
    X: PartialEq + Eq + Hash;

impl<X, Y> super::Evaluate<X, Y> for Evaluate<X, Y>
where
    X: PartialEq + Eq + Hash,
    Y: Clone + Copy,
{
    fn evaluate(&self, x: X) -> Y {
        *self.0.get(&x).expect("Out of bound: ValueMap")
    }
}
