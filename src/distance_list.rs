use crate::{measurer::Measurer, node::{Node, ItemInfo}};

pub struct DList<V, M: Measurer<V>>
{
    root: Option<Box<Node<V, M>>>,
    measurer: M,
}

impl<V, Me> DList<V, Me>
where
    Me: Measurer<V>,
{
    pub fn new(measurer: Me) -> Self {
        Self {
            root: None,
            measurer,
        }
    }
}

impl<V, M> DList<V, M>
where
    M: Measurer<V>,
{
    pub fn insert(&mut self, index: usize, val: V) {
        match self.root.take() {
            Some(box_to_node) => {
                self.root = Some(Node::insert(index, val, box_to_node, &self.measurer))
            }
            None => {
                let len = self.measurer.measure(&val);
                self.root = Some(Box::new(Node::new(val, len)))
            }
        }
    }

    pub fn append(&mut self, val: V) {
        self.insert(self.size(), val);
    }

    pub fn size(&self) -> usize {
        match &self.root {
            Some(root) => root.total_count,
            None => 0,
        }
    }

    pub fn length(&self) -> M::Measure {
        match &self.root {
            Some(root) => root.total_length,
            None => self.measurer.nil(),
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<ItemInfo<V, M::Measure>> {
        match self.root {
            Some(ref box_to_node) => Node::search_by_index(index, box_to_node, &self.measurer),
            None => None,
        }
    }

    pub fn get_by_distance(&self, distance: M::Measure) -> Option<ItemInfo<V, M::Measure>> {
        match self.root {
            Some(ref box_to_node) => Node::search_by_distance(distance, box_to_node, &self.measurer),
            None => None,
        }
    }

    pub fn delete(&mut self, index: usize) {
        match self.root.take() {
            Some(box_to_node) => self.root = Node::delete(index, box_to_node, &self.measurer),
            None => return,
        }
    }
}
