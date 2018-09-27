pub struct SimpleHeap<T> {
    data : Vec<T>
}

fn parent(index : usize) -> usize {
    if index != 0 {
        return index / 2;
    }else {
        return 0;
    }
}

fn left(index : usize) -> usize {
    if index != 0 {
        return 2 * index;
    }else {
        return 1;
    }
}

fn right(index : usize) -> usize {
    if index != 0 {
        return 2 * index + 1;
    }else {
        return 2;
    }
}

impl<T : PartialOrd> SimpleHeap<T> {
    pub fn new() -> SimpleHeap<T> {
        return SimpleHeap { data : Vec::new()};
    }

    pub fn min(&self) -> Option<&T> {
        let s = self.data.len();
        if s == 0 {
            return None;
        }else {
            return Some(&self.data[0]);
        }
    }

    fn exch(&mut self,i : usize,j : usize) -> (){
        self.data.swap(i,j);
    }

    pub fn add_data(&mut self,x : T) -> () {
        self.data.push(x);
        //self.data.swap(0,self.data.len()-1);
        //SimpleHeap::min_heapify(self,);
    }

    pub fn build_heap(&mut self) -> () {
        println!("size is {}",self.data.len());
        let s = self.data.len()/2;
        for i in (0..s).rev() {
            println!("i is {}",i);
            unsafe { SimpleHeap::min_heapify(self, i,0); }
        }
    }


    unsafe fn min_heapify(&mut self,passed_index : usize,offset : usize) -> () {
        let mut local_index = passed_index;
        while local_index < self.data.len() {

            let mut current_min_index = local_index;
            let current_min_val = &mut self.data[local_index] as *mut T;

            if left(local_index) < self.data.len() {
                let left_val = &mut self.data[left(local_index)] as *mut T;
                if *left_val < *current_min_val {
                    self.data.swap(local_index, left(local_index));
                    current_min_index = left(local_index);
                }
            }

            if right(local_index) < self.data.len() {
                let right_val = &mut self.data[right(local_index)] as *mut T;
                if *right_val < *current_min_val {
                    self.data.swap(local_index, right(local_index));
                    current_min_index = right(local_index);

                }
            }

//            if right(local_index) <= self.data.len() && current_min_val > &mut self.data[right(local_index)] {
//                self.data.swap(current_min_index,right(local_index));
//                current_min_val = self.data[local_index];
//                current_min_index = right(local_index);
//            }

            if current_min_index == local_index {
                break;
            }

            local_index = current_min_index;

        }

    }

    pub fn visit(&self,f : &Fn(&T) -> ())  {
        for x in &self.data {
            f(&x);
        }
    }



}