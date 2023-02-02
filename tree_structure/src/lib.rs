struct Node<'a> {
    value: usize,
    left: &'a Node<'a>,
    right: &'a Node<'a>,
}

pub struct AVLTree<'a> {
    nodes: Vec<Node<'a>>
}

impl AVLTree<'_> {
    pub fn new() -> Self {
        let nodes = Vec::new();

        AVLTree { nodes }
    }

    pub fn search(value: usize) -> usize {

    }

    pub fn insert(value: usize) -> usize {

    }

    pub fn delete(value: usize) {

    }
}

#[cfg(test)]
mod tests {
    use crate::AVLTree;

    #[test]
    fn test1() {
        let tree = AVLTree::new();
    }
}