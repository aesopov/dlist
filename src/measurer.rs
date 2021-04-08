use std::{fmt::Debug, ops::{Add, Sub}};

pub trait Measurer<V> {
    type Measure: Add<Output = Self::Measure> + Sub<Output = Self::Measure> + Eq + Ord + Copy + Debug;
    fn nil(&self) -> Self::Measure;
    fn measure(&self, value: &V) -> Self::Measure;
}
