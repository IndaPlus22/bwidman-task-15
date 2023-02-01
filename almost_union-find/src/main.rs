// Solution to Kattis problem "Almost Union-Find"
use std::io;
use std::io::prelude::*;

struct UnionFind {
    parents: Vec<usize>, // Parent of every number (0..n)
    children: Vec<Vec<usize>>, // Children of every number
    size: Vec<usize>, // Size of subtree
    sum: Vec<u64>, // Sum of elements in subtree
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parents: (0..size).collect(),
            children: vec![Vec::with_capacity(0); size],
            size: vec![1; size],
            sum: (1u64..=(size as u64)).collect(),
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

        if p_root == q_root { // p & q are already in the same set
            return;
        }

        // Move the set q is inside of as a branch under the root of the set p is under
        self.parents[q_root] = p_root;

        // Add the root of the q-set as a child of the root of the p-set
        self.children[p_root].push(q_root);

        // Add q's size and sum to p_root's sum
        self.size[p_root] += self.size[q_root];
        self.sum[p_root] += self.sum[q_root];
        eprintln!("{}: size: {}, sum: {}", q_root + 1, self.size[p_root], self.sum[p_root]);
    }
    
    // Operation 2
    fn move_number(&mut self, p: usize, q: usize) {
        let p_root = self.find(p);
        let q_root = self.find(q);

        if p_root == q_root { // p & q are already in the same set
            return;
        }

        if self.parents[p] != p && self.children[p].len() > 0 { // p is not the root
            // Set p's first child's parent as p's parent
            self.parents[self.children[p][0]] = self.parents[p];
        }

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
        
        // Add size and value of p to q_root's size and sum
        self.size[q_root] += 1;
        self.sum[q_root] += (p + 1) as u64;
        eprintln!("{}: size: {}, sum: {}", q_root + 1, self.size[q_root], self.sum[q_root]);
        
        // If p had children, set the first one's size and sum to p's, excluding p
        if self.children[p].len() > 0 {
            let first_child = self.children[p][0];
            self.size[first_child] = self.size[p] - 1;
            self.sum[first_child] = self.sum[p] - (p + 1) as u64;
        }

        // Remove p's children
        self.children[p].clear();
    }
    
    // Operation 3
    // Prints the number of elements and their sum
    fn size_sum(&mut self, p: usize) {
        let root = self.find(p);

        eprint!("Size, sum: ");
        println!("{} {}", self.size[root], self.sum[root]);
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
            3 => union_find.size_sum(parameters[1] - 1),
            _ => (),
        }
    }
}
