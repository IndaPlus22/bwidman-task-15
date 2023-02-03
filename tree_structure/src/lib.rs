use std::rc::Rc;

type ChildNode<T> = Option<Rc<Node<T>>>;

pub struct Node<T> where
T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    key: T,
    left: ChildNode<T>,
    right: ChildNode<T>,
}

pub struct AVLTree<T> where
T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    root: ChildNode<T>
}

impl<T> AVLTree<T> where
T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    // Searches for node representing key.
    // Returns None if node does not exist.
    pub fn search(&self, key: T) -> Option<&Rc<Node<T>>> {
        let mut node = self.root.as_ref();

        while node.is_some() && node.unwrap().key != key {
            if key < node.unwrap().key {
                node = node.unwrap().left.as_ref();
            } else {
                node = node.unwrap().right.as_ref();
            }
        }
        return node;
    }

    pub fn insert(&mut self, key: T) {
        let mut node = self.root.clone();
        let mut parent: Option<Rc<Node<T>>> = None;

        // Find suitable empty spot in tree
        while node.is_some() {
            parent = node;
            if key < node.unwrap().key {
                node = &mut node.unwrap().left;
            } else {
                node = &mut node.unwrap().right;
            }
        }

        if parent.is_none() { // Empty tree
            self.root = Some(Rc::new(Node { key, left: None, right: None }));
        } else if key < parent.unwrap().key {
            parent.unwrap().left = Some(Rc::new(Node { key, left: None, right: None }));
        }
    }

    pub fn delete(&mut self, key: T) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tree: AVLTree<u32> = AVLTree::new();
    }
}