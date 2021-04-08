extern crate dlist;

use dlist::{DList, Measurer};

struct MeasurerU32;

impl Measurer<u32> for MeasurerU32 {
    type Measure = u64;
    fn nil(&self) -> u64 {
        0
    }

    fn measure(&self, value: &u32) -> u64 {
        *value as u64
    }
}

#[test]
fn get_dist() {
    let mut dl = DList::new(MeasurerU32);
    dl.append(0);
    dl.append(1);
    dl.append(2);
    dl.append(3);
    dl.append(4);
    dl.append(5);
    println!("{:?}", dl.get_by_distance(0));
    if let Some(ii) = dl.get_by_distance(0) {
        assert_eq!(ii.index, 0);
        assert_eq!(ii.item, &0);
        assert_eq!(ii.inner_distance, 0);
        assert_eq!(ii.outer_distance, 0);
    }
}
