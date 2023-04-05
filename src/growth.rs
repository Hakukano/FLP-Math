use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use super::{evaluate::Evaluate, progress::Progress};

/// Growing value by level
///
/// # Generic
///
/// - `Lvl` - Level type
/// - `Exp` - Exp type
/// - `Eval` - Evaluation
#[derive(Deserialize, Serialize)]
pub struct Growth<Lvl, Exp, Eval>
where
    Eval: Evaluate<Lvl, Exp>,
{
    lvl: PhantomData<Lvl>,
    evaluation: Eval,
    progress: Progress<Exp>,
}

macro_rules! impl_growth {
    ($(($l:ty, $p:ty),)*) => {
        $(
        impl<Eval> Growth<$l, $p, Eval>
        where
            Eval: Evaluate<$l, $p>,
        {
            pub fn new(evaluation: Eval, progress: Progress<$p>) -> Self {
                Self {
                    lvl: PhantomData,
                    evaluation,
                    progress,
                }
            }

            pub fn progress(&self) -> &Progress<$p> {
                &self.progress
            }

            pub fn progress_mut(&mut self) -> &mut Progress<$p> {
                &mut self.progress
            }

            pub fn apply_level(&mut self, level: $l) {
                let new_max = self.evaluation.evaluate(level);
                self.progress.set_max(new_max);
            }
        }
        )*
    };
}

impl_growth!(
    (u8, u8),
    (u8, u16),
    (u8, u32),
    (u8, u64),
    (u16, u8),
    (u16, u16),
    (u16, u32),
    (u16, u64),
    (u32, u8),
    (u32, u16),
    (u32, u32),
    (u32, u64),
);
