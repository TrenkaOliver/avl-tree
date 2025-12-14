use std::fmt::Debug;

pub struct Node<K, V>
where K: Ord + Debug {
    pub key: K,
    pub value: V,
    pub height: u32,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
}

impl<K, V> Debug for Node<K, V> 
where K: Ord + Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
        .field("key", &self.key)
        .field("left", &self.left)
        .field("right", &self.right)
        .finish()
    }
}

impl<K, V> Node<K, V>
where K: Ord + Debug {
    pub fn new(key: K, value: V) -> Node<K, V> {
        Node { key, value, height: 1, left: None, right: None }
    }

    pub fn get_bf(self: &Box<Self>) -> i32 {
        let hl = self.left.as_ref().map(|n| n.height).unwrap_or(0) as i32;
        let hr = self.right.as_ref().map(|n| n.height).unwrap_or(0) as i32;

        hl - hr
    }

    pub fn balance(self: &mut Box<Node<K, V>>) {
        let bf = self.get_bf();

        if bf.abs() <= 1 {
            self.update_height();
            return;
        }

        let is_left = bf > 0;

        let child = if is_left {
            self.left.as_mut().unwrap()
        } else {
            self.right.as_mut().unwrap()
        };

        let prev_is_left = child.get_bf() > 0;

        //left left
        if is_left && prev_is_left {
            self.rotate_right();
        }
        
        //right right
        else if !is_left && !prev_is_left {
            self.rotate_left();
        }

        //left right
        else if is_left && !prev_is_left {
            child.rotate_left();
            self.rotate_right();
        }

        //right, left
        else {
            child.rotate_right();
            self.rotate_left();
        }
    }

    pub fn update_height(self: &mut Box<Self>) {
        self.height = 1 + u32::max(
            self.left.as_ref().map(|n| n.height ).unwrap_or(0),
            self.right.as_ref().map(|n| n.height ).unwrap_or(0),
        )
    }

    pub fn rotate_left(self: &mut Box<Self>) {
        let mut b = self.right.take().expect("rotate_left: called on node without right child");
        let b_left = b.left.take();
        
        self.right = b_left;

        let mut old_parent = std::mem::replace(self, b);
        old_parent.update_height();

        self.left = Some(old_parent);
        self.update_height();
    }

    pub fn rotate_right(self: &mut Box<Self>) {
        let mut b = self.left.take().expect("rotate_right: called on node without left child");
        let b_right = b.right.take();

        self.left = b_right;

        let mut old_parent = std::mem::replace(self, b);
        old_parent.update_height();

        self.right = Some(old_parent);
        self.update_height();
    }

    //min node, new node to replace original
    pub fn extract_min_rec(mut self: Box<Self>) -> (Option<Box<Self>>, Option<Box<Self>>) {
        if let Some(left) = self.left.take() {
            let (min_node, new_left) = Self::extract_min_rec(left);
            self.left = new_left;
            self.balance();
            (min_node, Some(self))
        } else {
            let right = self.right.take();
            (Some(self), right)
        }
    }
}