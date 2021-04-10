use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

pub trait Measurer<V> {
    type Measure: Add<Output = Self::Measure>
        + Sub<Output = Self::Measure>
        + Eq
        + Ord
        + Copy
        + Debug;
    fn nil(&self) -> Self::Measure;
    fn measure(&self, value: &V) -> Self::Measure;
}

pub struct DefaultMeasurer<T>
where
    T: Default + Add<Output = T> + Sub<Output = T> + Eq + Ord + Copy + Debug,
{
    _t: std::marker::PhantomData<T>,
}

impl<T: Default + Add<Output = T> + Sub<Output = T> + Eq + Ord + Copy + Debug> DefaultMeasurer<T> {
    pub fn new() -> DefaultMeasurer<T> {
        Self {_t: std::marker::PhantomData}
    }
}

impl<T: Default + Add<Output = T> + Sub<Output = T> + Eq + Ord + Copy + Debug> Measurer<T>
    for DefaultMeasurer<T>
{
    type Measure = T;
    fn nil(&self) -> T {
        T::default()
    }
    fn measure(&self, value: &T) -> T {
        *value
    }
}
