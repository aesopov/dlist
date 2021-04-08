use std::cmp::{self, Ordering};

use crate::measurer::Measurer;

#[derive(Debug, PartialEq)]
pub struct ItemInfo<'a, V, M> {
    pub item: &'a V,
    pub index: usize,
    pub outer_distance: M,
    pub inner_distance: M,
}

pub(crate) struct Node<V, M: Measurer<V>> {
    value: V,
    left: Option<Box<Node<V, M>>>,
    right: Option<Box<Node<V, M>>>,
    height: u16,
    pub total_count: usize,
    pub total_length: M::Measure,
}

impl<V, M: Measurer<V>> Node<V, M> {
    pub fn new(value: V, length: M::Measure) -> Self {
        Self {
            value,
            total_count: 1,
            height: 1,
            total_length: length,
            left: None,
            right: None,
        }
    }

    fn height(node: &Option<Box<Self>>) -> u16 {
        node.as_ref().map_or(0, |x| x.height)
    }

    fn total_count(node: &Option<Box<Self>>) -> usize {
        node.as_ref().map_or(0, |x| x.total_count)
    }

    fn total_length(node: &Option<Box<Self>>, measurer: &M) -> M::Measure {
        node.as_ref().map_or(measurer.nil(), |x| x.total_length)
    }

    fn update_node(&mut self, measurer: &M) {
        self.height = cmp::max(Self::height(&self.left), Self::height(&self.right)) + 1;
        self.total_count = Self::total_count(&self.left) + Self::total_count(&self.right) + 1;
        self.total_length = Self::total_length(&self.left, measurer)
            + Self::total_length(&self.right, measurer)
            + measurer.measure(&self.value);
    }

    fn rotate_right(mut root: Box<Self>, measurer: &M) -> Box<Self> {
        let mut new_root_box = root.left.unwrap();
        root.left = new_root_box.right.take();
        root.update_node(measurer);
        new_root_box.right = Some(root);
        new_root_box.update_node(measurer);
        return new_root_box;
    }

    fn rotate_left(mut root: Box<Self>, measurer: &M) -> Box<Self> {
        let mut new_root_box = root.right.unwrap();
        root.right = new_root_box.left.take();
        root.update_node(measurer);
        new_root_box.left = Some(root);
        new_root_box.update_node(measurer);
        return new_root_box;
    }

    fn rotate_left_successor(mut root: Box<Self>, measurer: &M) -> Box<Self> {
        let left = root.left.unwrap();
        if Self::height(&left.left) < Self::height(&left.right) {
            let rotated = Self::rotate_left(left, measurer);
            root.left = Some(rotated);
            root.update_node(measurer);
        } else {
            root.left = Some(left);
        }
        Self::rotate_right(root, measurer)
    }

    fn rotate_right_successor(mut root: Box<Self>, measurer: &M) -> Box<Self> {
        let right = root.right.unwrap();
        if Self::height(&right.left) > Self::height(&right.right) {
            let rotated = Self::rotate_right(right, measurer);
            root.right = Some(rotated);
            root.update_node(measurer);
        } else {
            root.right = Some(right)
        }
        Self::rotate_left(root, measurer)
    }

    fn diff_of_successors_height(root: &Box<Self>) -> i32 {
        let l = Self::height(&root.left);
        let r = Self::height(&root.right);
        (l as i32) - (r as i32)
    }

    fn rotate_if_necessary(root: Box<Self>, measurer: &M) -> Box<Self> {
        let diff = Self::diff_of_successors_height(&root);
        if -1 <= diff && diff <= 1 {
            return root;
        }
        match diff {
            2 => Self::rotate_left_successor(root, measurer),
            -2 => Self::rotate_right_successor(root, measurer),
            _ => unreachable!(),
        }
    }

    fn insert_in_successor(
        index: usize,
        val: V,
        successor: Option<Box<Self>>,
        measurer: &M,
    ) -> Option<Box<Self>> {
        Some(match successor {
            Some(succ) => Self::insert(index, val, succ, measurer),
            None => {
                let len = measurer.measure(&val);
                Box::new(Node::<V, M>::new(val, len))
            }
        })
    }

    pub fn insert(index: usize, val: V, mut root: Box<Self>, measurer: &M) -> Box<Self> {
        let left_count = Self::total_count(&root.left);
        match left_count.cmp(&index) {
            Ordering::Less => {
                root.right = Self::insert_in_successor(
                    index - left_count - 1,
                    val,
                    root.right.take(),
                    measurer,
                )
            }
            Ordering::Greater | Ordering::Equal => {
                root.left = Self::insert_in_successor(index, val, root.left.take(), measurer)
            }
        }
        root.update_node(measurer);
        return Self::rotate_if_necessary(root, measurer);
    }

    pub fn search_by_index<'a>(
        index: usize,
        root: &'a Box<Self>,
        measurer: &M,
    ) -> Option<ItemInfo<'a, V, M::Measure>> {
        let left_count = Self::total_count(&root.left);
        match left_count.cmp(&index) {
            Ordering::Equal => Some(ItemInfo {
                item: &root.value,
                index: left_count,
                outer_distance: Self::total_length(&root.left, measurer),
                inner_distance: measurer.nil(),
            }),
            Ordering::Less => {
                let mut r = root.right.as_ref().map_or(None, |succ| {
                    Self::search_by_index(index - left_count - 1, succ, measurer)
                })?;
                r.index += left_count + 1;
                r.outer_distance = r.outer_distance + Self::total_length(&root.left, measurer) + measurer.measure(&root.value);
                Some(r)
            }
            Ordering::Greater => root
                .left
                .as_ref()
                .map_or(None, |succ| Self::search_by_index(index, succ, measurer)),
        }
    }

    pub(crate) fn search_by_distance<'a>(
        distance: M::Measure,
        root: &'a Box<Self>,
        measurer: &'a M,
    ) -> Option<ItemInfo<'a, V, M::Measure>> {
        let left_length = Self::total_length(&root.left, measurer);
        let left_count = Self::total_count(&root.left);
        if left_count > 0 && distance == measurer.nil() {
            let left_child = root.left.as_ref().unwrap();
            return Self::search_by_distance(distance, &left_child, measurer);
        }
        match left_length.cmp(&distance) {
            Ordering::Less | Ordering::Equal => {
                let distance = distance - left_length;
                let root_length = measurer.measure(&root.value);
                if distance < root_length {
                    let result = ItemInfo {
                        item: &root.value,
                        index: Self::total_count(&root.left),
                        outer_distance: left_length,
                        inner_distance: distance,
                    };
                    return Some(result);
                }
                let mut result = root.right.as_ref().map_or(None, |succ| {
                    Self::search_by_distance(distance - root_length, succ, measurer)
                })?;
                result.index += Self::total_count(&root.left) + 1;
                result.outer_distance = result.outer_distance + left_length + root_length;
                println!("result\t{:?} {:?}", result.index, result.inner_distance);
                Some(result)
            }
            Ordering::Greater => root.left.as_ref().map_or(None, |succ| {
                Self::search_by_distance(distance, succ, measurer)
            }),
        }
    }

    fn updated_node(mut root: Box<Self>, measurer: &M) -> Box<Self> {
        root.update_node(measurer);
        Self::rotate_if_necessary(root, measurer)
    }

    fn drop_min_from_left(
        mut root: Box<Self>,
        left: Box<Self>,
        measurer: &M,
    ) -> (Option<Box<Self>>, Box<Self>) {
        let (new_left, min) = Self::drop_min(left, measurer);
        root.left = new_left;
        (Some(Self::updated_node(root, measurer)), min)
    }

    fn drop_min(mut root: Box<Self>, measurer: &M) -> (Option<Box<Self>>, Box<Self>) {
        match root.left.take() {
            Some(left) => Self::drop_min_from_left(root, left, measurer),
            None => (root.right.take(), root),
        }
    }

    fn combine_two_subtrees(l: Box<Self>, r: Box<Self>, measurer: &M) -> Box<Self> {
        let (remaining_tree, min) = Self::drop_min(r, measurer);
        let mut new_root = min;
        new_root.left = Some(l);
        new_root.right = remaining_tree;
        Self::updated_node(new_root, measurer)
    }

    fn delete_root(mut root: Box<Self>, measurer: &M) -> Option<Box<Self>> {
        match (root.left.take(), root.right.take()) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (Some(l), Some(r)) => Some(Self::combine_two_subtrees(l, r, measurer)),
        }
    }

    pub fn delete(
        index: usize,
        mut root: Box<Self>,
        measurer: &M,
    ) -> Option<Box<Self>> {
        let left_count = Self::total_count(&root.left);
        match left_count.cmp(&index) {
            Ordering::Equal => return Self::delete_root(root, measurer),
            Ordering::Less => {
                if let Some(succ) = root.right.take() {
                    root.right = Self::delete(index - left_count - 1, succ, measurer);
                    return Some(Self::updated_node(root, measurer));
                }
            }
            Ordering::Greater => {
                if let Some(succ) = root.left.take() {
                    root.left = Self::delete(index, succ, measurer);
                    return Some(Self::updated_node(root, measurer));
                }
            }
        }
        return Some(root);
    }
}
