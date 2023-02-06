use std::sync::{Arc, Mutex};

type ChildNode<T> = Option<Arc<Mutex<Node<T>>>>;

fn create_child<T>(key: T) -> ChildNode<T>
where T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    Some(Arc::new(Mutex::new(Node { key, parent: None, left: None, right: None })))
}

pub struct Node<T>
where T: Copy + Clone + PartialEq + PartialOrd + Eq + Ord {
    key: T,
    parent: ChildNode<T>,
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
            if n.lock().unwrap().key == key {
                break;
            }

            if key < n.lock().unwrap().key {
                node = n.lock().unwrap().left.clone();
            } else {
                node = n.lock().unwrap().right.clone();
            }
        }
        return node;
    }

    // Inserts a new node with this key into the tree
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

        node.unwrap().lock().unwrap().parent = parent.clone();

        // Set node as child of parent
        match parent {
            None => self.root = create_child(key), // Empty tree
            Some(p) => {
                if key < p.lock().unwrap().key {
                    p.lock().unwrap().left = create_child(key);
                } else {
                    p.lock().unwrap().right = create_child(key);
                }
            }
        }

        // Balance
    }

    fn minimum(mut x: ChildNode<T>) -> ChildNode<T> {
        while let Some(left) = x.clone().unwrap().lock().unwrap().left.clone() {
            x = left.lock().unwrap().left.clone();
        }
        return x;
    }

    fn successor(mut x: ChildNode<T>) -> ChildNode<T> {
        if x.clone().unwrap().lock().unwrap().right.is_some() {
            return Self::minimum(x.clone().unwrap().lock().unwrap().right.clone());
        }
        let mut y = x.clone().unwrap().lock().unwrap().parent.clone();

        while y.is_some() && x.clone().unwrap().lock().unwrap().key == y.clone().unwrap().lock().unwrap().right.clone().unwrap().lock().unwrap().key {
            x = y.clone();
            y = y.unwrap().lock().unwrap().parent.clone();
        }
        return y;
    }

    // Replace u with v (helper function for delete)
    fn shift_nodes(&mut self, u: ChildNode<T>, v: ChildNode<T>) {
        match u.clone().unwrap().lock().unwrap().parent.clone() {
            None => self.root = v, // u was the root
            Some(parent) => {
                if u.unwrap().lock().unwrap().key == parent.lock().unwrap().left.clone().unwrap().lock().unwrap().key {
                    parent.lock().unwrap().left = v.clone();
                } else {
                    parent.lock().unwrap().right = v.clone();
                }

                if let Some(v) = v {
                    v.lock().unwrap().parent = Some(parent);
                }
            }
        }
    }

    // Removes node with this key from the tree
    pub fn delete(&mut self, key: T) {
        if let Some(del_node) = self.search(key) { // Node exists
            if del_node.lock().unwrap().left.is_none() { // No left and maybe no right
                self.shift_nodes(Some(del_node.clone()), del_node.lock().unwrap().right.clone());
            } else if del_node.lock().unwrap().right.is_none() { // Left but no right
                self.shift_nodes(Some(del_node.clone()), del_node.lock().unwrap().left.clone());
            } else { // Both left and right
                let successor = Self::successor(Some(del_node.clone()));

                // Successor not direct child of deleted node
                if successor.clone().unwrap().lock().unwrap().parent.clone().unwrap().lock().unwrap().key != del_node.lock().unwrap().key {
                    // Make successor's right child take it's place
                    self.shift_nodes(successor.clone(), successor.clone().unwrap().lock().unwrap().right.clone());

                    successor.clone().unwrap().lock().unwrap().right = del_node.lock().unwrap().right.clone();
                    successor.clone().unwrap().lock().unwrap().right.clone().unwrap().lock().unwrap().parent = successor.clone();
                }
                self.shift_nodes(Some(del_node.clone()), successor.clone());
                successor.clone().unwrap().lock().unwrap().left = del_node.lock().unwrap().left.clone();
                successor.clone().unwrap().lock().unwrap().left.clone().unwrap().lock().unwrap().parent = successor.clone();
            }
        }
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