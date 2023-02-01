// Solution to Kattis problem "Almost Union-Find"
use std::io;
use std::io::prelude::*;

struct UnionFind {
    parents: Vec<usize>, // Parent of every number (0..n)
    children: Vec<Vec<usize>> // Children of every number
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let mut parents = vec![0; size];
        for i in 0..size {
            parents[i] = i;
        }

        UnionFind {
            parents,
            children: vec![Vec::with_capacity(0); size]
        }
    }

    // Find representative of set
    fn find(&mut self, index: usize) -> usize {
        // If parent is itself, it's the root/representative
        if self.parents[index] == index {
            return index;
        }
        return self.find(self.parents[index]);
    }
    
    // Operation 1
    fn union(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);

        // Move the set q is inside of as a branch under the root of the set p is under
        self.parents[q_root] = p_root;

        // Add the root of the q-set as a child of the root of the p-set
        self.children[p_root].push(q_root);
    }
    
    // Operation 2
    fn move_number(&mut self, p: usize, q: usize) {
        let q_root = self.find(q);
        // Set p's parent to the root of q
        self.parents[p] = q_root;
        
        // Add p as a child of the root of q
        self.children[q_root].push(p);
        
        // Set p's children's parent to the first child
        // and add the other children as children to the first child
        for p_child in 0..self.children[p].len() {
            let first_child = self.children[p][0];
            
            let child_index = self.children[p][p_child];
            self.parents[child_index] = first_child;
            eprintln!("Set {}'s parent to {}", child_index + 1, first_child + 1);
            
            if child_index != first_child {
                self.children[first_child].push(child_index);
                eprintln!("Add {} as child to {}", child_index + 1, first_child + 1);
            }
        }

        // Remove p's children
        self.children[p].clear();
    }
    
    // Operation 3
    // Prints the number of child elements and their sum (including p)
    fn size_sum(&self, p: usize) -> (usize, usize) {
        // Return value and size of 1 if arrived at a leaf (end of tree)
        if self.children[p].len() == 0 {
            return (1, p + 1);
        }

        let mut size = 1;
        let mut sum = p + 1; // Start with current root's value
        eprintln!("Root: {sum}");

        // Recurse through every child and add it's size and sum
        for child in 0..self.children[p].len() {
            let index = self.children[p][child];
            let (child_size, child_sum) = self.size_sum(index);
            size += child_size;
            sum += child_sum;
            eprintln!("sum += {child_sum}");
        }

        return (size, sum);
    }
}

fn parse_line(line: String) -> Vec<usize> {
    return line.split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
}

fn main() {
    // Get all input
    let mut lines = io::stdin()
        .lock()
        .lines()
        .map(|line| line.ok().unwrap());
    
    // First line: n m
    // n: size of the set
    // m: number of commands
    let first_line = parse_line(lines.next().unwrap());
    let n = first_line[0];

    let mut union_find = UnionFind::new(n);

    // Perform operations on the union-find structure, line by line
    for line in lines {
        let parameters = parse_line(line);
        let operation = parameters[0];

        match operation {
            1 => union_find.union(parameters[1] - 1, parameters[2] - 1),
            2 => union_find.move_number(parameters[1] - 1, parameters[2] - 1),
            3 => {
                let root = union_find.find(parameters[1] - 1);
                let (size, sum) = union_find.size_sum(root);
                eprint!("Size, sum: ");
                println!("{} {}", size, sum);
            },
            _ => (),
        }
    }
}
