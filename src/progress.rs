use std::fmt::Display;

use num_traits::{CheckedAdd, CheckedSub, FromPrimitive};
use serde::{Deserialize, Serialize};

/// Progression
///
/// # Generic
///
/// - `T` - value type
#[derive(Deserialize, Serialize)]
pub struct Progress<T> {
    current: T,
    max: T,
}

impl<T> Progress<T>
where
    T: Clone + Copy + Display + PartialOrd + CheckedAdd + CheckedSub + FromPrimitive,
{
    /// Constructor
    ///
    /// # Arguments
    ///
    /// - `current` - Current value
    /// - `max` - Max value
    ///
    /// # Panic
    ///
    /// If `current` is larger than `max`
    pub fn new(current: T, max: T) -> Self {
        if current > max {
            panic!("Invalid progress: {current} > {max}");
        }
        Self { current, max }
    }

    pub fn current(&self) -> &T {
        &self.current
    }

    pub fn set_current(&mut self, n: T) {
        if n > self.max {
            self.current = self.max;
        } else {
            self.current = n;
        }
    }

    pub fn max(&self) -> &T {
        &self.max
    }

    pub fn set_max(&mut self, n: T) {
        if self.current > n {
            self.current = n;
        }
        self.max = n;
    }

    /// add n to the progress, return the rest more than max
    pub fn add(&mut self, n: T) -> T {
        let added = self.current + n;
        if added > self.max {
            self.current = self.max;
            added - self.max
        } else {
            self.current = added;
            T::from_u8(0).unwrap()
        }
    }

    /// sub n from the progress, return the rest less than min
    pub fn sub(&mut self, n: T) -> T {
        if self.current < n {
            let rest = n - self.current;
            self.current = T::from_u8(0).unwrap();
            rest
        } else {
            self.current = self.current - n;
            T::from_u8(0).unwrap()
        }
    }
}
