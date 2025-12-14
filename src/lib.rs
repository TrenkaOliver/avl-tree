mod node;

use crate::node::*;

use std::fmt::Debug;

pub struct AvlTree<K, V>
where K: Ord + Debug {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> AvlTree<K, V>
where K: Ord + Debug {
    pub fn from(key: K, value: V) -> AvlTree<K, V> {
        AvlTree { root: Some(Box::new(Node::new(key, value)))}
    }

    pub fn new() -> AvlTree<K, V> {
        AvlTree { root: None }
    }

    pub fn print(&self) {
        if self.root.is_none() {return;}
        let r = self.root.as_ref().unwrap();
        let mut layers = vec![format!("{:?}({})", r.key, r.height)];
        let depth = 1; //start with 1st layer (one with max 2 elements)
        Self::print_recursive(&r, &mut layers, depth);

        for layer in layers {
            println!("{layer}");
        }
    }

    fn print_recursive(parent: &Node<K, V>, layers: &mut Vec<String>, depth: usize) {
        if layers.len() <= depth {
            layers.push(String::new());
        }

        if let Some(left_node) = &parent.left {
            layers[depth].push_str(&format!("{:?}({}) ", left_node.key, left_node.height));
            Self::print_recursive(&left_node, layers, depth + 1);
        } else {
            layers[depth].push_str("     ");
        }

        if let Some(right_node) = &parent.right {
            layers[depth].push_str(&format!("{:?}({}) ", right_node.key, right_node.height));
            Self::print_recursive(&right_node, layers, depth + 1); 
        } else {
            layers[depth].push_str("     ");
        }
    }

    pub fn find_value(&self, key: &K) -> Option<&V> {
        let mut current_node = self.root.as_ref();
        
        while let Some(node) = current_node {
            if *key == node.key {
                return Some(&node.value);
            } else if *key < node.key {
                current_node = node.left.as_ref();
            } else {
                current_node = node.right.as_ref();
            }
        }

        None
    }

    pub fn insert(&mut self, key: K, value: V) -> bool {
        let new_node = Box::new(Node::new(key, value));
        Self::insert_rec(self.root.as_mut().unwrap(), new_node)
    }

    fn insert_rec(parent: &mut Box<Node<K, V>>, new_node: Box<Node<K, V>>) -> bool {
        let new_place = match new_node.key.cmp(&parent.key) {
            std::cmp::Ordering::Less => &mut parent.left,
            std::cmp::Ordering::Greater => &mut parent.right,
            std::cmp::Ordering::Equal => return false,
        };

        if let Some(new_parent) = new_place {
            if !Self::insert_rec(new_parent, new_node) {
                return false;
            }
        } else {
            *new_place = Some(new_node);
        }        
        
        parent.balance();
        true
    }

    pub fn remove(&mut self, key: &K) -> bool {
        let (new_root, matched) = Self::remove_rec(self.root.take(), key);
        self.root = new_root;

        if !matched {
            return false;
        }

        self.root.as_mut().map(Node::balance);
        
        true
    }

    fn remove_rec(node: Option<Box<Node<K, V>>>, key: &K) -> (Option<Box<Node<K, V>>>, bool) {
        let mut node = match node {
            Some(n) => n,
            None => return (None, false),
        };

        let child = match key.cmp(&node.key) {
            std::cmp::Ordering::Less => &mut node.left,
            std::cmp::Ordering::Greater => &mut node.right,
            std::cmp::Ordering::Equal => {
                match (node.left.take(), node.right.take()) {
                    (None, None) => return (None, true),
                    (left, None) => return (left, true),
                    (None, right) => return (right, true),
                    (left, Some(right)) => {
                        let (new_node, new_right) = right.extract_min_rec();
                        let mut new_node = new_node.expect("cannot be None");
                        
                        new_node.left = left;
                        new_node.right = new_right;

                        return (Some(new_node), true);
                    },
                }
            },
        };

        let (new_child, matched) = Self::remove_rec(child.take(), key);
        *child = new_child;
        
        if matched {
            node.balance();
        }

        (Some(node), matched)
    }

}