#![feature(test)]

extern crate dlist;
extern crate test;

use std::rc::Rc;

use dlist::{DList, Measurer};
use lipsum::lipsum;
use test::Bencher;

struct MeasurerU32;
struct MeasurerU64;
struct MeasurerString;
struct MeasurerStringRc;

impl Measurer<u32> for MeasurerU32 {
    type Measure = u64;
    fn nil(&self) -> u64 {
        0
    }

    fn measure(&self, value: &u32) -> u64 {
        *value as u64
    }
}

impl Measurer<u64> for MeasurerU64 {
    type Measure = u64;
    fn nil(&self) -> u64 {
        0
    }

    fn measure(&self, value: &u64) -> u64 {
        *value
    }
}

impl Measurer<String> for MeasurerString {
    type Measure = usize;
    fn nil(&self) -> usize {
        0
    }

    fn measure(&self, value: &String) -> usize {
        value.len()
    }
}

impl Measurer<Rc<String>> for MeasurerStringRc {
    type Measure = usize;
    fn nil(&self) -> usize {
        0
    }

    fn measure(&self, value: &Rc<String>) -> usize {
        value.len()
    }
}

fn add_n(n: usize, b: &mut Bencher) {
    let mut lines = Vec::new();
    for _i in 0..n {
        let mut lorem = lipsum(25);
        lorem.shrink_to_fit();
        lines.push(Rc::new(lorem));
    }
    b.iter(|| {
        let mut dl = DList::new(MeasurerStringRc);
        for (i, s) in lines.iter().enumerate() {
            dl.insert(i, s.clone());
        }
    });
}

#[bench]
fn add_1(b: &mut Bencher) {
    add_n(1, b);
}

#[bench]
fn add_1000(b: &mut Bencher) {
    add_n(1000, b);
}

#[bench]
fn add_100000(b: &mut Bencher) {
    add_n(100000, b);
}
