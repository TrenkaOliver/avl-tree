use std::fmt::Debug;
use avl_tree::*;

fn main() {
    let mut nums = 1..=10;

    let mut bst = AvlTree::new(nums.next().unwrap(), 0);


    for num in nums {
        bst.insert(num, 0);
    }

    bst.print();
    bst.remove(&8);
    bst.print();
}

