use std::mem;

pub struct List<T> {
    head: Link<T>,
}

pub enum Link<T> {
    Empty,
    More(Box<Node<T>>)
}

struct Node<T> {
    elem : T,
    next : Link<T>
}

impl<T> List<T>  {
    pub fn new() -> List<T> {
        let new_node = List { head : Link::Empty};
        return new_node;
    }

    pub fn push(&mut self,x : T) -> () {
        let new_node = Node {
            elem : x,
            next : mem::replace(&mut self.head,Link::Empty)
        };

        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                return None;
            },
            Link::More(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                result = Some(node.elem);
                return result;
            }

        }
    }

    pub fn iter(&self, _f : &Fn(&T) -> ()) -> () {
        fn iter_local<T>(link : &Link<T>,_g : &Fn(&T) -> ()) -> () {
            match link {
                Link::Empty => {
                    print!("{}","{}")
                },
                Link::More(ref x) => {
                    _g(&(*x).elem);
                    iter_local(&(*x).next,_g);
                }
            }
        }

        iter_local(&self.head, _f);

    }
}
