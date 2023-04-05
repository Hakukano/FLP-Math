use std::{cmp::Ordering, fmt::Display};

use num_traits::{CheckedAdd, CheckedRem, CheckedSub, FromPrimitive};
use serde::{Deserialize, Serialize};

/// A wrappable bound
///
/// # Generic
///
/// - `T` - value type
#[derive(Deserialize, Serialize)]
pub struct Bound<T> {
    pub min: T,
    pub max: T,
    wrap: bool,
}

impl<T> Bound<T>
where
    T: Clone + Copy + Display + PartialOrd + CheckedAdd + CheckedSub + CheckedRem + FromPrimitive,
{
    /// Constructor
    ///
    /// # Arguments
    ///
    /// - `min` - Lower bound
    /// - `max` - Upper bound
    /// - `wrap`
    ///     - `true` - wrap from the opposite bound
    ///     - `false` - stop at the bound
    ///
    /// # Panic
    ///
    /// If `min` is larger than `max`
    pub fn new(min: T, max: T, wrap: bool) -> Self {
        if min > max {
            panic!("Invalid bound: {min} > {max}");
        }
        Self { min, max, wrap }
    }

    pub fn check(&self, n: T) -> Ordering {
        if n < self.min {
            Ordering::Less
        } else if n > self.max {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Apply this bound to a value
    ///
    /// # Arguments
    ///
    /// - `n` - the value to apply to
    ///
    /// # Return
    ///
    /// The value after applying bound
    ///
    /// # Examples
    ///
    /// ```
    /// use flp_math::bound::Bound;
    /// let bound = Bound::new(3, 7, true);
    /// assert_eq!(bound.apply(10), 5);
    /// assert_eq!(bound.apply(12), 7);
    /// assert_eq!(bound.apply(13), 3);
    /// assert_eq!(bound.apply(0), 5);
    /// assert_eq!(bound.apply(-2), 3);
    /// assert_eq!(bound.apply(-3), 7);
    /// ```
    pub fn apply(&self, n: T) -> T {
        let zero = T::from_u8(0).unwrap();
        let one = T::from_u8(1).unwrap();
        match self.check(n) {
            Ordering::Less => {
                if self.wrap {
                    let range = self.max - self.min + one;
                    let diff = (self.min - n) % range;
                    if diff == zero {
                        self.min
                    } else {
                        self.max - diff + one
                    }
                } else {
                    self.min
                }
            }
            Ordering::Greater => {
                if self.wrap {
                    let range = self.max - self.min + one;
                    let diff = (n - self.max) % range;
                    if diff == zero {
                        self.max
                    } else {
                        self.min + diff - one
                    }
                } else {
                    self.max
                }
            }
            _ => n,
        }
    }
}
