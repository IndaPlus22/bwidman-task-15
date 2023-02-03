use std::sync::{Arc, Mutex};

type ChildNode<T> = Option<Arc<Mutex<Node<T>>>>;

fn create_child<T>(key: T) -> ChildNode<T>
where T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    Some(Arc::new(Mutex::new(Node { key, left: None, right: None })))
}

pub struct Node<T>
where T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    key: T,
    left: ChildNode<T>,
    right: ChildNode<T>,
}

pub struct AVLTree<T>
where T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    root: ChildNode<T>
}

impl<T> AVLTree<T>
where T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    // Searches for node representing key.
    // Returns None if node does not exist.
    pub fn search(&self, key: T) -> ChildNode<T> {
        let mut node = self.root.clone();

        while let Some(n) = node.clone() {
            while n.lock().unwrap().key != key {
                if key < n.lock().unwrap().key {
                    node = n.lock().unwrap().left.clone();
                } else {
                    node = n.lock().unwrap().right.clone();
                }
            }
        }
        return node;
    }

    pub fn insert(&mut self, key: T) {
        let mut node = self.root.clone();
        let mut parent: ChildNode<T> = None;

        // Find suitable empty spot in tree
        while let Some(n) = node.clone() {
            parent = node;
            if key < n.lock().unwrap().key {
                node = n.lock().unwrap().left.clone();
            } else {
                node = n.lock().unwrap().right.clone();
            }
        }

        match parent {
            None => self.root = create_child(key), // Empty tree
            Some(p) => {
                if key < p.lock().unwrap().key {
                    p.lock().unwrap().left = create_child(key);
                } else if key > p.lock().unwrap().key {
                    p.lock().unwrap().right = create_child(key);
                }
            }
        }
    }

    pub fn delete(&mut self, _key: T) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let _tree: AVLTree<u32> = AVLTree::new();
    }
}