use std::fmt::Debug;

fn main() {
    let mut nums = 1..=10;

    let mut bst = AvlTree::new(nums.next().unwrap(), 0);


    for num in nums {
        bst.insert(num, 0);
    }

    bst.print();
    bst.remove(&23);
    bst.print();
}

struct Node<K, V>
where K: Eq + Ord + Debug {
    key: K,
    value: V,
    height: u32,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Debug for Node<K, V> 
where K: Eq + Ord + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
        .field("key", &self.key)
        .field("left", &self.left)
        .field("right", &self.right)
        .finish()
    }
}

impl<K, V> Node<K, V>
where K: Eq + Ord + Debug {
    pub fn new(key: K, value: V) -> Node<K, V> {
        Node { key, value, height: 1, left: None, right: None }
    }
}

struct AvlTree<K, V>
where K: Eq + Ord + Debug {
    pub root: Option<Box<Node<K, V>>>,
}

impl<K, V> AvlTree<K, V>
where K: Eq + Ord + Debug {
    pub fn new(key: K, value: V) -> AvlTree<K, V> {
        AvlTree { root: Some(Box::new(Node::new(key, value)))}
    }

    pub fn from(root: Node<K, V>) -> AvlTree<K, V> {
        AvlTree {root: Some(Box::new(root))}
    }

    pub fn print(&self) {
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
        Self::insert_rec(self.root.as_mut().unwrap(), new_node).is_some()
    }

    fn insert_rec(parent: &mut Box<Node<K, V>>, new_node: Box<Node<K, V>>) -> Option<bool> {
        if parent.key == new_node.key {return None}
        
        let is_left = new_node.key < parent.key;
        let (new_place, other_child) = if is_left {
            (&mut parent.left, &mut parent.right)
        } else {
            (&mut parent.right, &mut parent.left)
        };
        
        let prev_is_left = if let Some(new_parent) = new_place {
            Self::insert_rec(new_parent, new_node)?
        } else {
            *new_place = Some(new_node);
            Self::update_height(parent);
            return Some(is_left);
        };


        let h1 = new_place.as_ref().map(|n| n.height).unwrap_or(0) as i32;
        let h2 = other_child.as_ref().map(|n| n.height).unwrap_or(0) as i32;

        let bf = h1 - h2;

        if bf.abs() <= 1 {
            Self::update_height(parent);
            return Some(is_left);
        }


        //left left
        if is_left && prev_is_left {
            Self::rotate_right(parent);
        }
        
        //right right
        else if !is_left && !prev_is_left {
            Self::rotate_left(parent);
        }

        //left right
        else if is_left && !prev_is_left {
            Self::rotate_left(new_place.as_mut().unwrap());
            Self::rotate_right(parent);
        }

        //right, left
        else {
            Self::rotate_right(new_place.as_mut().unwrap());
            Self::rotate_left(parent);
        }

        Some(is_left)

    }

    pub fn remove(&mut self, key: &K) -> bool {
        let (new_root, matched) = Self::remove_rec(self.root.take(), key);
        self.root = new_root;

        if !matched {
            return false;
        }

        self.root.as_mut().map(Self::balance_node);
        
        true
    }

    fn remove_rec(node: Option<Box<Node<K, V>>>, key: &K) -> (Option<Box<Node<K, V>>>, bool) {
        let mut node = match node {
            Some(n) => n,
            None => return (None, false),
        };

        if *key == node.key {
            match (node.left.take(), node.right.take()) {
                (None, None) => return (None, true),
                (left, None) => return (left, true),
                (None, right) => return (right, true),
                (left, Some(right)) => {
                    let (new_node, new_right) = Self::extract_min_rec(right);
                    let mut new_node = new_node.expect("cannot be None");
                    
                    new_node.left = left;
                    new_node.right = new_right;

                    return (Some(new_node), true);
                },
            }
        }

        let child = if *key < node.key {
            &mut node.left
        } else {
            &mut node.right
        };

        let (new_child, matched) = Self::remove_rec(child.take(), key);
        *child = new_child;
        
        if matched {
            Self::balance_node(&mut node);
            (Some(node), true)
        } else {
            (Some(node), false)
        }
    }

    //min node, new node to replace original
    fn extract_min_rec(mut node: Box<Node<K, V>>) -> (Option<Box<Node<K, V>>>, Option<Box<Node<K, V>>>) {
        if let Some(left) = node.left.take() {
            let (min_node, new_left) = Self::extract_min_rec(left);
            node.left = new_left;
            Self::balance_node(&mut node);
            (min_node, Some(node))
        } else {
            let right = node.right.take();
            (Some(node), right)
        }
    }

    fn get_bf(node: &Box<Node<K, V>>) -> i32 {
        let hl = node.left.as_ref().map(|n| n.height).unwrap_or(0) as i32;
        let hr = node.right.as_ref().map(|n| n.height).unwrap_or(0) as i32;

        hl - hr
    }

    fn balance_node(node: &mut Box<Node<K, V>>) {
        let bf = Self::get_bf(&node);

        if bf.abs() <= 1 {
            Self::update_height(node);
            return;
        }

        let is_left = bf > 0;

        let child = if is_left {
            node.left.as_mut().unwrap()
        } else {
            node.right.as_mut().unwrap()
        };

        let prev_is_left = Self::get_bf(&child) > 0;

        //left left
        if is_left && prev_is_left {
            Self::rotate_right(node);
        }
        
        //right right
        else if !is_left && !prev_is_left {
            Self::rotate_left(node);
        }

        //left right
        else if is_left && !prev_is_left {
            Self::rotate_left(child);
            Self::rotate_right(node);
        }

        //right, left
        else {
            Self::rotate_right(child);
            Self::rotate_left(node);
        }
    }

    fn update_height(node: &mut Box<Node<K, V>>) {
        node.height = 1 + u32::max(
            node.left.as_ref().map(|n| n.height ).unwrap_or(0),
            node.right.as_ref().map(|n| n.height ).unwrap_or(0),
        )
    }

    fn rotate_left(parent: &mut Box<Node<K, V>>) {
        let mut b = parent.right.take().expect("rotate_left: called on node without right child");
        let b_left = b.left.take();
        
        parent.right = b_left;

        let mut old_parent = std::mem::replace(parent, b);
        Self::update_height(&mut old_parent);

        parent.left = Some(old_parent);
        Self::update_height(parent);
    }

    fn rotate_right(parent: &mut Box<Node<K, V>>) {
        let mut b = parent.left.take().expect("rotate_right: called on node without left child");
        let b_right = b.right.take();

        parent.left = b_right;

        let mut old_parent = std::mem::replace(parent, b);
        Self::update_height(&mut old_parent);

        parent.right = Some(old_parent);
        Self::update_height(parent);
    }

}