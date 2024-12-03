use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

register_solver!(2022, 7, Solver);
use common::prelude::*;

pub struct Solver;

#[derive(Debug)]
struct Dir {
    name: String,
    children: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Weak<RefCell<Dir>>>,
    size: usize,
}

impl MonoDaySolver for Solver {
    fn solve(&self, input: &str) -> (PartResult, PartResult) {
        let dirs = get_dirs(input);

        let res1 = dirs
            .iter()
            .filter_map(|n| {
                let b = n.borrow();
                (b.size <= 100000).then_some(b.size)
            })
            .sum::<usize>();

        let total_size = dirs[0].borrow().size;
        let target_size = total_size - (70000000 - 30000000);
        let res2 = dirs
            .iter()
            .filter_map(|n| {
                let b = n.borrow();
                (b.size >= target_size).then_some(b.size)
            })
            .min()
            .unwrap();

        (PartResult::new(res1), PartResult::new(res2))
    }
}

fn get_dirs(input: &str) -> Vec<Rc<RefCell<Dir>>> {
    let mut nodes = vec![];
    let mut cur_node: Option<Rc<RefCell<Dir>>> = None;
    for l in input
        .split('\n')
        // In practice "$ ls" and "dir" lines are not useful as all
        // information is given by "cd" and file size entries
        .filter(|&l| !(l == "$ ls" || l.starts_with("dir")))
    {
        let (cmd, payload) = l.rsplit_once(' ').unwrap();

        match cmd {
            "$ cd" => {
                if payload == ".." {
                    cur_node = Some(
                        cur_node
                            .unwrap()
                            .borrow()
                            .parent
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap(),
                    );
                    continue;
                }

                // Avoid reinsertion of node
                if let Some(n) = &cur_node {
                    let b = n.borrow();
                    if b.children.iter().any(|n| n.borrow().name == payload) {
                        continue;
                    }
                }

                let node = Rc::new(RefCell::new(Dir {
                    name: payload.to_string(),
                    children: vec![],
                    parent: cur_node.clone().map(|n| Rc::downgrade(&n)),
                    size: 0,
                }));
                nodes.push(Rc::clone(&node));
                if let Some(n) = &cur_node {
                    n.borrow_mut().children.push(Rc::clone(&node))
                }
                cur_node = Some(node);
            }
            file_size => {
                let file_size = file_size.parse::<usize>().unwrap();

                let node = cur_node.clone().unwrap();

                node.borrow_mut().size += file_size;

                let mut node = node.borrow().parent.clone();

                // Propagate size to parent dirs
                while let Some(n) = node {
                    let n = n.upgrade().unwrap();
                    let mut b = n.borrow_mut();
                    b.size += file_size;
                    node = b.parent.clone();
                }
            }
        }
    }
    nodes
}
