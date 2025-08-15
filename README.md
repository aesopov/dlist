# DList

[![Crates.io](https://img.shields.io/crates/v/dlist.svg)](https://crates.io/crates/dlist)
[![Documentation](https://docs.rs/dlist/badge.svg)](https://docs.rs/dlist)
[![License](https://img.shields.io/badge/license-MIT%2FUnlicense-blue.svg)](https://github.com/misupov/dlist)

A high-performance list data structure built on an AVL tree that supports efficient distance-based queries and logarithmic-time operations.

## Overview

DList is designed for scenarios where you need to store elements with measurable dimensions and quickly search for elements by their cumulative distance from the beginning. All list mutations and indexing operations run in **O(log n)** time complexity.

## Key Features

- **Distance-based queries**: Find elements by their cumulative distance in O(log n)
- **Efficient indexing**: Access elements by index in O(log n)
- **Balanced structure**: Built on AVL tree for guaranteed performance
- **Generic design**: Works with any type that implements the `Measurer` trait
- **Memory efficient**: Compact representation with automatic balancing

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
dlist = "0.1.4"
```

### Basic Usage

```rust
use dlist::{DList, DefaultMeasurer};

// Create a new DList for storing integers
let mut dlist = DList::new(DefaultMeasurer::new());

// Add elements
dlist.append(10);
dlist.append(20);
dlist.append(30);

// Access by index
let item_info = dlist.get_by_index(1).unwrap();
assert_eq!(*item_info.item, 20);
assert_eq!(item_info.index, 1);

// Access by distance
let item_info = dlist.get_by_distance(15).unwrap();
assert_eq!(*item_info.item, 20);
assert_eq!(item_info.outer_distance, 10);
assert_eq!(item_info.inner_distance, 5);
```

## Use Cases

### Text Editor Implementation

DList excels at text editor scenarios where you need to map file offsets to line/column positions:

```rust
use dlist::{DList, DefaultMeasurer};

// Create a DList to track line lengths
let mut dlist = DList::new(DefaultMeasurer::new());

// Assume we have parsed a file into line lengths
let line_lengths = vec![42, 38, 55, 29, 67]; // Line lengths in bytes

for length in line_lengths {
    dlist.append(length);
}

// Find line/column for a specific file offset
let offset = 125; // Byte offset in file
if let Some(line_info) = dlist.get_by_distance(offset) {
    let line_number = line_info.index;           // Zero-based line index
    let line_length = *line_info.item;           // Length of this line
    let line_start = line_info.outer_distance;   // Start offset of the line
    let column = line_info.inner_distance;       // Column within the line
    
    println!("Offset {} is at line {}, column {}", offset, line_number, column);
}

// Direct line access
if let Some(line_info) = dlist.get_by_index(2) {
    println!("Line 2 has {} characters", line_info.item);
}
```

### Custom Measurements

Implement the `Measurer` trait for custom distance calculations:

```rust
use dlist::{DList, Measurer};

struct Point {
    x: f64,
    y: f64,
}

struct EuclideanMeasurer;

impl Measurer<Point> for EuclideanMeasurer {
    type Measure = f64;
    
    fn nil(&self) -> f64 {
        0.0
    }
    
    fn measure(&self, point: &Point) -> f64 {
        (point.x * point.x + point.y * point.y).sqrt()
    }
}

let mut path = DList::new(EuclideanMeasurer);
path.append(Point { x: 3.0, y: 4.0 }); // Distance: 5.0
path.append(Point { x: 1.0, y: 0.0 }); // Distance: 1.0

// Find point at cumulative distance 3.0
let result = path.get_by_distance(3.0);
```

## API Reference

### Core Types

- **`DList<T>`**: The main list structure
- **`ItemInfo<T>`**: Information about an item including its position and distances
- **`Measurer<T>`**: Trait for defining how to measure items
- **`DefaultMeasurer<T>`**: Built-in measurer for numeric types

### Key Methods

- **`append(item: T)`**: Add an item to the end of the list
- **`get_by_index(index: usize)`**: Retrieve item by its position
- **`get_by_distance(distance: M)`**: Find item by cumulative distance
- **`insert(index: usize, item: T)`**: Insert item at specific position
- **`remove(index: usize)`**: Remove item at specific position

## Performance

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Insert    | O(log n)       | O(1)            |
| Remove    | O(log n)       | O(1)            |
| Get by index | O(log n)    | O(1)            |
| Get by distance | O(log n) | O(1)            |
| Append    | O(log n)       | O(1)            |

## License

This project is dual-licensed under the [MIT License](LICENSE-MIT) and [Unlicense](UNLICENSE). You may choose either license for your use.

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests on [GitHub](https://github.com/misupov/dlist).
