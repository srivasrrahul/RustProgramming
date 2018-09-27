//Conclusion : Its next to impossible to write link list in unsafe rust
enum ListLink<T> {
    Empty,
    Next(Box<ListNode<T>>)
}
pub struct ListNode<T> {
    data : T,
    link : ListLink<T>
}

pub struct LinkList<T> {
    head : ListLink<T>
}

impl<T> ListNode<T> {
    pub fn new(x : T) -> ListNode<T> {
        return ListNode { data : x,link : ListLink::Empty}
    }
}


impl<T> LinkList<T> {
    pub fn new() -> LinkList<T> {
        return LinkList {head: ListLink::Empty};
    }

//    pub fn append(&mut self,x : T) -> LinkList<T> {
//        let mut new_node = ListNode::new(x);
//        let mut pointer = self.head;
//        let mut done = false;
//        while !done {
//            match pointer {
//                Empty => {
//                    //its empty
//                    self.head = new_node;
//                },
//                Next => {
//
//            }
//            }
//        }
//    }
}

