extern crate dlist;

use dlist::{DList, ItemInfo, Measurer};

struct StringMeasurer;

impl Measurer<String> for StringMeasurer {
    type Measure = usize;
    fn nil(&self) -> usize {
        0
    }

    fn measure(&self, value: &String) -> usize {
        value.len()
    }
}

#[test]
fn del_odd() {
    let mut dl = DList::new(StringMeasurer);
    dl.append("a".to_string());
    dl.append("b".to_string());
    dl.append("c".to_string());
    dl.append("d".to_string());
    dl.append("e".to_string());
    dl.append("f".to_string());
    dl.delete(0);
    dl.delete(1);
    dl.delete(2);
    assert_eq!(
        Some(ItemInfo {
            index: 0,
            item: &"b".to_string(),
            inner_distance: 0,
            outer_distance: 0,
        }),
        dl.get_by_index(0)
    );
    assert_eq!(
        Some(ItemInfo {
            index: 1,
            item: &"d".to_string(),
            inner_distance: 0,
            outer_distance: 1,
        }),
        dl.get_by_index(1)
    );
    assert_eq!(
        Some(ItemInfo {
            index: 2,
            item: &"f".to_string(),
            inner_distance: 0,
            outer_distance: 2,
        }),
        dl.get_by_index(2)
    );
}
