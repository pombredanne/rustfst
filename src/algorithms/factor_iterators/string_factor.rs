use crate::algorithms::FactorIterator;
use crate::semirings::Semiring;
use crate::semirings::{
    StringWeightLeft, StringWeightRestrict, StringWeightRight, StringWeightVariant,
};

pub struct StringFactorLeft {
    weight: StringWeightLeft,
    done: bool,
}

pub struct StringFactorRight {
    weight: StringWeightRight,
    done: bool,
}

pub struct StringFactorRestrict {
    weight: StringWeightRestrict,
    done: bool,
}

macro_rules! impl_string_factor {
    ($factor: ident, $semiring: ident) => {
        impl Iterator for $factor {
            type Item = ($semiring, $semiring);

            fn next(&mut self) -> Option<Self::Item> {
                if self.done() {
                    return None;
                }
                let l = self.weight.value.unwrap_labels();
                let l1 = vec![l[0]];
                let l2: Vec<_> = l.iter().skip(1).cloned().collect();
                self.done = true;
                Some((l1.into(), l2.into()))
            }
        }

        impl FactorIterator<$semiring> for $factor {
            fn new(weight: $semiring) -> Self {
                let done = match &weight.value {
                    StringWeightVariant::Infinity => true,
                    StringWeightVariant::Labels(l) => (l.len() <= 1),
                };
                Self { weight, done }
            }
            fn done(&self) -> bool {
                self.done
            }
        }
    };
}

impl_string_factor!(StringFactorLeft, StringWeightLeft);
impl_string_factor!(StringFactorRight, StringWeightRight);
impl_string_factor!(StringFactorRestrict, StringWeightRestrict);
