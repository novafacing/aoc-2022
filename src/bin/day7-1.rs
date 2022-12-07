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
            ["dir", name] => FSNode::Directory(name.to_string()),
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
                        println!("cd {}", subdir);
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
}
