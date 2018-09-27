use std::collections::BTreeSet;

pub struct TopN {
    n: usize,
    set : BTreeSet<i32>
}

impl TopN {
    pub fn new(s : usize) -> TopN {
        return TopN { n : s,set : BTreeSet::new()};
    }

    pub fn add(&mut self,val : i32) -> () {
        if self.set.len() < self.n {
            self.set.insert(val);
        }else {
            let & min_val = self.set.iter().next().unwrap();
            if val > min_val {
                self.set.insert(val);
                self.set.remove(&min_val);
            }
        }
    }

    pub fn getN(&self) -> () {
        let mut iter = self.set.iter();
        loop {
            match iter.next() {
                Some(x) => {
                    println!("{}",x)
                },
                None => {
                    break
                }
            }
        }
    }
}