/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // If the root is None, create a new node and set it as the root
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }

        // Otherwise, insert the value into the tree
        // find the correct place to insert the value
        let mut current = self.root.as_mut().unwrap();
        loop {
            match value.cmp(&current.value) {
                Ordering::Less => {
                    if current.left.is_none() {
                        current.left = Some(Box::new(TreeNode::new(value)));
                        return;
                    }
                    current = current.left.as_mut().unwrap();
                }
                Ordering::Greater => {
                    if current.right.is_none() {
                        current.right = Some(Box::new(TreeNode::new(value)));
                        return;
                    }
                    current = current.right.as_mut().unwrap();
                }
                Ordering::Equal => {
                    // If the value is already in the tree, do nothing
                    return;
                }
            }
        }
        
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let current = self.root.as_ref();
        if current.is_none() {
            return false;
        }

        let mut current = current.unwrap();
        loop {
            match value.cmp(&current.value) {
                Ordering::Less => {
                    if current.left.is_none() {
                        return false;
                    }
                    current = current.left.as_ref().unwrap();
                }
                Ordering::Greater => {
                    if current.right.is_none() {
                        return false;
                    }
                    current = current.right.as_ref().unwrap();
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if self.left.is_none() {
                    self.left = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.left.as_mut().unwrap().insert(value);
                }
            }
            Ordering::Greater => {
                if self.right.is_none() {
                    self.right = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.right.as_mut().unwrap().insert(value);
                }
            }
            Ordering::Equal => {
                // If the value is already in the tree, do nothing
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


