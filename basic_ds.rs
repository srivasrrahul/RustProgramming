
use std::collections::LinkedList;

struct Stack {
    data :  [i32;5],
    top : usize
}

impl Stack {
    fn new() -> Stack {
        let  st = Stack { data : [0;5],top : 0};
        return st;
    }

    pub fn push(&mut self,val : i32) -> () {

        self.data[self.top] = val;
        self.top = self.top+1;
    }

    pub fn pop(&self) -> i32 {
        return self.data[self.top-1];
    }


}

struct Queue<T> {
    data : LinkedList<T>
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        let q : Queue<T> = Queue { data : LinkedList::new()};
        return q;
    }

    pub fn add(&mut self,val : T) -> () {
        self.data.push_back(val);
    }

    pub fn remove(&mut self) -> Option<T> {
        return self.data.pop_back();
    }
}

fn main() {
    let mut st = Stack::new();
    st.push(10);
    println!("{}",st.pop());

    let mut st1 = Stack::new();
    st1.push(11);
    println!("{}",st1.pop());

    let mut q : Queue<i32> = Queue::new();
    q.add(10);
    println!("{}",q.remove().unwrap());
}