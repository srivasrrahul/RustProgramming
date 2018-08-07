pub struct BinaryTree<T> {
    data : T,
    left : Option<Box<BinaryTree<T>>>,
    right : Option<Box<BinaryTree<T>>>
}

pub fn test() -> i32 {
    return 2;
}

impl<T> BinaryTree<T> {
    pub fn new1(value : T) -> Box<BinaryTree<T>>{
        return Box::new(BinaryTree { data : value,left : None,right : None});
    }

    pub fn add_left(node : &mut BinaryTree<T>,value : T) -> &Option<Box<BinaryTree<T>>> {
        match node.left {
            Some(ref mut left_tree) => {
                return BinaryTree::add_left(&mut *left_tree,value);
            },
            None => {
                node.left = Some(BinaryTree::new1(value));
                return &node.left;
            }
        }
    }

    pub fn add_right(node : &mut BinaryTree<T>,value : T) -> &Option<Box<BinaryTree<T>>> {
        match node.right {
            Some(ref mut right_tree) => {
                return BinaryTree::add_right(&mut *right_tree,value);
            },
            None => {
                node.right = Some(BinaryTree::new1(value));
                return &node.right;
            }
        }
    }

    pub fn visit(&self,f : &Fn(&T) -> ()) -> () {
        f(&self.data);
        match self.left {
            Some(ref x) => {
                x.visit(f);
            },
            None => {

            }
        }
        match &self.right {
            Some(x) => {
                x.visit(f);
            },
            None => {

            }
        }
    }
}

//fn main() {
//
//}
