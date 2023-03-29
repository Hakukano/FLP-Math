pub struct Progress<T> {
    current: T,
    max: T,
}

macro_rules! impl_progress {
    ($($t:ty,)*) => {
        $(
        impl Progress<$t> {
            pub fn new(current: $t, max: $t) -> Self {
                if current > max {
                    panic!("Invalid progress: {current} > {max}");
                }
                Self { current, max }
            }

            pub fn current(&self) -> &$t {
                &self.current
            }

            pub fn set_current(&mut self, n: $t) {
                self.current = n.min(self.max);
            }

            pub fn max(&self) -> &$t {
                &self.max
            }

            pub fn set_max(&mut self, n: $t) {
                self.current = self.current.min(n);
                self.max = n;
            }

            /// add n to the progress, return the rest more than max
            pub fn add(&mut self, n: $t) -> $t {
                let added = self.current + n;
                if added > self.max {
                    self.current = self.max;
                    added - self.max
                } else {
                    self.current = added;
                    0
                }
            }

            /// sub n from the progress, return the rest less than min
            pub fn sub(&mut self, n: $t) -> $t {
                if self.current < n {
                    let rest = n - self.current;
                    self.current = 0;
                    rest
                } else {
                    self.current -= n;
                    0
                }
            }
        }
        )*
    };
}

impl_progress!(u8, u16, u32, u64,);
