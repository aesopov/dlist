# dlist
List data structure based on AVL tree. It can store elements which have dimension and quickly search for elements by distance from 0.
The list mutations and indexing are O(logN).

## Use cases
### Text Editors
DList can be used to split text data into lines and calculate line/column numbers based on absolute file offset:
```rust

let mut dlist = DList::new(DefaultMeasurer::new() as DefaultMeasurer<usize>);

let line_lengths: Vec<usize> = ... // let's say we have parsed some text file and now have a vector containing the length of each line (in bytes)

for i in 0..line_lengths.len() {
    dlist.append(line_lengths[i]);
}

let line_info = dl.get_by_index(100).unwrap();
assert_eq!(line_info.index, 100);               // a line index. Since we use get_by_index(n), it should always be equal to n.
assert_eq!(line_info.item, 50);                 // in this case an item is of type usize and contains the length of the text line.
assert_eq!(line_info.outer_distance, 1000);     // an absolute file offset of the first character in the line.
assert_eq!(line_info.inner_distance, 0);        // always 0 for get_by_index


let line_info = dl.get_by_distance(10000).unwrap();
assert_eq!(line_info.index, 100);               // a zero-based line index for the distance (i.e. file offset).
assert_eq!(line_info.item, 50);                 // a length of the line which contains this distance.
assert_eq!(line_info.outer_distance, 1000);     // an absolute file offset of the first character in the line.
assert_eq!(line_info.inner_distance, 34);       // an offset inside the slice, i.e. zero-based column offset.

```
