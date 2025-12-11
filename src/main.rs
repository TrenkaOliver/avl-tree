use std::{fmt::Debug};

fn main() {
    let mut nums = 1..=50;
    let values: Vec<i32> = (51..=100).collect();

    let mut bst = BinaryTree::new(nums.next().unwrap(), 0);


    for num in nums {
        println!("iteration num: {num}");
        bst.insert(num, values[num - 1]);
        bst.print();

        println!("\nend of iteration\n-----------------------------------------")
    }

    println!("{:?}", bst.find_value(&68));
}

struct Node<K, V>
where K: Eq, K: Ord, K: Debug {
    key: K,
    value: V,
    height: u32,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V>
where K: Eq, K: Ord, K: Debug {
    pub fn new(key: K, value: V) -> Node<K, V> {
        Node { key, value, height: 1, left: None, right: None }
    }
}

struct BinaryTree<K, V>
where  K: Eq, K: Ord, K: Debug {
    pub root: Box<Node<K, V>>,
}

impl<K, V> BinaryTree<K, V>
where K: Eq, K: Ord, K: Debug {
    pub fn new(key: K, value: V) -> BinaryTree<K, V> {
        BinaryTree { root: Box::new(Node::new(key, value))}
    }

    pub fn from(root: Node<K, V>) -> BinaryTree<K, V> {
        BinaryTree {root: Box::new(root)}
    }

    pub fn print(&self) {
        let mut layers = vec![format!("{:?}({})", self.root.key, self.root.height)];
        let depth = 1; //start with 1st layer (one with max 2 elements)
        Self::print_recursive(&self.root, &mut layers, depth);

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
        let mut current_node = Some(&self.root);
        
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

    pub fn insert(&mut self, key: K, value: V) {
        let new_node = Box::new(Node::new(key, value));
        Self::insert_recursive(&mut self.root, new_node);
    }

    fn insert_recursive(parent: &mut Box<Node<K, V>>, new_node: Box<Node<K, V>>) -> Option<bool> {
        if parent.key == new_node.key {return None}
        
        let is_left = new_node.key < parent.key;
        let (new_place, other_child) = if is_left {
            (&mut parent.left, &mut parent.right)
        } else {
            (&mut parent.right, &mut parent.left)
        };
        
        let prev_is_left = if let Some(new_parent) = new_place {
            Self::insert_recursive(new_parent, new_node)?
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