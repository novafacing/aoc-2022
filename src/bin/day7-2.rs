use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::{stdin, Read},
    rc::Rc,
};

use id_tree::{InsertBehavior, Node, Tree, TreeBuilder};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum FSNode {
    Directory(String),
    File((String, usize)),
}

impl FSNode {
    fn new(s: &str) -> Self {
        match s.split(" ").collect::<Vec<_>>().as_slice() {
            ["dir", name] => FSNode::Directory(name.to_string().replace("/", "")),
            [size, name] => FSNode::File((name.to_string(), size.parse().unwrap())),
            _ => panic!("Invalid node"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Ls,
    Cd(String),
}

impl Instruction {
    fn new(s: &str) -> Self {
        match s.split(" ").collect::<Vec<_>>().as_slice() {
            ["$", "ls"] => Instruction::Ls,
            ["$", "cd", name] => Instruction::Cd(name.to_string()),
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug)]
enum Item {
    FSNode(FSNode),
    Instruction(Instruction),
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut instr_idx = 0;
    let mut tree: Tree<FSNode> = TreeBuilder::new()
        .with_node_capacity(input.lines().count())
        .build();
    let mut curid = tree
        .insert(
            Node::new(FSNode::Directory("/".to_string())),
            InsertBehavior::AsRoot,
        )
        .unwrap();

    let rootid = curid.clone();

    input
        .lines()
        .enumerate()
        .filter(|(i, _)| i > &0)
        .map(|(_, l)| l.trim())
        .map(|l| match l.chars().nth(0).unwrap() {
            '$' => Item::Instruction(Instruction::new(l)),
            _ => Item::FSNode(FSNode::new(l)),
        })
        .group_by(|i| match i {
            Item::FSNode(_) => instr_idx,
            Item::Instruction(_) => {
                instr_idx += 1;
                instr_idx
            }
        })
        .into_iter()
        .for_each(|(_, group)| {
            let mut batch = group.collect::<Vec<_>>();
            let instr = batch.remove(0);
            let instr = match instr {
                Item::Instruction(ref i) => i,
                _ => panic!("Invalid instruction"),
            };
            match instr {
                Instruction::Ls => batch.iter().for_each(|e| match e {
                    Item::FSNode(n) => {
                        // Add the fsnode to the current cursor node
                        tree.insert(Node::new(n.clone()), InsertBehavior::UnderNode(&curid))
                            .unwrap();
                    }
                    _ => panic!("Invalid node"),
                }),
                Instruction::Cd(ref name) => match name.as_str() {
                    ".." => {
                        // Pop up in traversal
                        let parent = tree.get(&curid).unwrap().parent().unwrap();
                        curid = parent.clone();
                    }
                    subdir => {
                        // Add the new fsnode to the current cursor node if it isn't there
                        // Traverse to the new fsnode
                        let children = tree.children_ids(&curid).unwrap();
                        curid = children
                            .into_iter()
                            .filter(|c| {
                                let node = tree.get(c).unwrap();
                                match node.data() {
                                    FSNode::Directory(ref n) => n == subdir,
                                    _ => false,
                                }
                            })
                            .map(|c| c.clone())
                            .next()
                            .unwrap_or_else(|| {
                                tree.insert(
                                    Node::new(FSNode::Directory(subdir.to_string())),
                                    InsertBehavior::UnderNode(&curid),
                                )
                                .unwrap()
                            });
                    }
                },
            }
        });

    let mut stack = vec![rootid];
    let mut sizes = HashMap::new();
    let mut seenfiles: HashSet<Vec<String>> = HashSet::new();
    while let Some(id) = stack.pop() {
        let node = tree.get(&id).unwrap();
        let mut path = tree
            .ancestors(&id)
            .unwrap()
            .into_iter()
            .map(|n| match n.data() {
                FSNode::Directory(ref n) => n,
                _ => panic!("Invalid node"),
            })
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .map(|n| n.to_string())
            .collect::<Vec<_>>();
        match node.data() {
            FSNode::Directory(ref n) => {
                path.push(n.to_string());
                if !sizes.contains_key(&path) {
                    sizes.insert(path.clone(), 0);
                }
            }
            FSNode::File((ref n, ref s)) => {
                path.push(n.to_string());

                if seenfiles.contains(&path) {
                    continue;
                }

                seenfiles.insert(path.clone());

                while path.len() > 1 {
                    path.pop();
                    if !sizes.contains_key(&path) {
                        sizes.insert(path.clone(), 0);
                    }
                    let size = sizes.get_mut(&path).unwrap();
                    *size += s;
                }
            }
        }
        let child_ids = tree.children_ids(&id).unwrap();
        child_ids.for_each(|c| stack.push(c.clone()));
    }

    let sizeof_root = sizes.get(&vec!["/".to_string()]).unwrap();
    let needed_space = 30000000 - (70000000 - sizeof_root);
    // Find the smallest dir larger than needed_space
    let smallest = sizes
        .iter()
        .filter(|(_, v)| **v >= needed_space)
        .min_by_key(|(_, v)| **v)
        .unwrap();
    println!("{:?}", smallest);
}
