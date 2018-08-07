use std::cmp;
pub struct BSTNode<T> {
    data : T,
    left : Option<Box<BSTNode<T>>>,
    right : Option<Box<BSTNode<T>>>
}

impl<T : PartialOrd> BSTNode<T> {
    pub fn new(d : T) -> Box<BSTNode<T>> {
        return Box::new(BSTNode { data : d,left : None,right : None})
    }

    pub fn add_data(&mut self,value : T) -> () {
        if value < self.data {
            match self.left {
                Some(ref mut left_tree) => {
                    left_tree.add_data(value);
                },
                None => {
                    self.left = Some(BSTNode::new(value))
                }
            }
        }else {
            match self.right {
                Some(ref mut right_tree) => {
                    right_tree.add_data(value);
                },
                None => {
                    self.right = Some(BSTNode::new(value))
                }
            }
        }
    }

    pub fn visit(&self,f : &Fn(&T) -> ()) {
        match self.left {
            Some(ref left_tree) => left_tree.visit(f),
            None => {}
        }
        f(&self.data);
        match self.right {
            Some(ref right_tree) => right_tree.visit(f),
            None => {}
        }
    }
}

