//use std::mem;
extern crate rand;

use rand::Rng;
use std::env;
use std::collections::LinkedList;

mod binary_tree;
//use binary_tree;

mod simple_lst;

//mod simple_lst1;
mod basic_sorts;

mod simple_heap;

mod binary_search_tree;

mod top_n;

mod ls;

//mod top_n;
fn random_shuffle(slice : &mut [i32]) -> () {
    let mut rng = rand::thread_rng();

    for i in 0..slice.len()-1 {
        let rand_idx = rng.gen_range(i+1,slice.len());
        println!("{}",rand_idx);
        slice.swap(i,rand_idx);
    }
}


fn print(x : &i32) -> () {
    println!("{}",x);
}
fn main() {

    //println!("Hello, world!");
//    let mut x = [1,2,3,4,5];
//    random_shuffle(&mut x);
//    println!("======");
//    for xi in &x {
//        println!("{}",xi);
//    }
 //let b_tree = binary_tree::new1(10);
//    let mut t = binary_tree::BinaryTree::new1(10);
//    binary_tree::BinaryTree::add_left(&mut t,20);
//    binary_tree::BinaryTree::add_right(&mut t,30);
//    binary_tree::BinaryTree::add_left(&mut t,40);
//    binary_tree::BinaryTree::add_right(&mut t,50);
//    t.visit(&print);
//    let mut bst_root = binary_search_tree::BSTNode::new(100);
//    bst_root.add_data(50);
//    bst_root.add_data(150);
//    bst_root.add_data(25);
//    bst_root.add_data(125);
//    bst_root.visit(&print);
    //let mut simple_lst = simple_lst::SimpleList::new(10);
//    let mut lst : simple_lst::List<i32>= simple_lst::List::new();
//    lst.push(10);
//    lst.push(20);
//    println!("{:?}",lst.pop());
//    println!("{:?}",lst.pop());
//    println!("{:?}",lst.pop());
//    lst.iter(&print);
    //binary_tree::BinaryTree::visit(print);
//    let mut arr : [i32;5] = [3,2,4,1,5];
//    let x : usize = arr.len()-1;
//    println!("{}",x);
//    //basic_sorts::sort_bubble(& mut arr);
//    let inversion_count = basic_sorts::merge_sort(& mut arr,x);
//    for i in &arr {
//        println!("{}",i);
//    }
//
//    println!("Inversion count : {}",inversion_count);
//    let mut heap : simple_heap::SimpleHeap<i32> = simple_heap::SimpleHeap::new();
//    heap.add_data(40); //0
//    heap.add_data(20); //1
//    heap.add_data(10); //2
//    heap.add_data(3);  //3
//    heap.add_data(2);  //4
//    heap.add_data(1);  //5
//    heap.add_data(100);//6
//    heap.add_data(7);  //7
//    heap.build_heap();    //8
//    //heap.heap_sort();
//    heap.visit(&print);
//    let mut vec = Vec::new();
//    vec.push("rahul");
//    vec.push("rohit");
//
//    for x in &vec {
//        println!("{}",x);
//    }
//    let args : Vec<String> = env::args().collect();
//    //println!("{:?}",args);
//
//    let mut vec : Vec<i32>= Vec::new();
//    for i in 1..args.len() {
//        println!("{}",args[i]);
//        vec.push(args[i].parse().unwrap())
//    }
//
//    vec.sort();
//    println!("{:?}",vec);
//
//    let mut m = 0;
//    {
//
//
//
//    }
//
//    let mean = |item: &i32| {
//        m += item;
//        println!("Mean is {}", m);
//    };
//
//    vec.iter().map(mean);
//    let mut topN = top_n::TopN::new(3);
//    let mut rng = rand::thread_rng();
//    for i in 0..10000 {
//        let r = rng.gen_range(0,10000);
//        println!("{}",r);
//        topN.add(r);
//
//    }
//    topN.add(10);
//    topN.add(20);
//    topN.add(30);
//    topN.add(40);
//    topN.add(50);
//    topN.add(50);
//    println!("*******");
//    topN.getN();



    //let z = m/vec.len();
    //{
        //println!("Mean is {}", m);
    //}
    //let x = ls::list_lines("/Users/rasrivastava/a.txt");
//    let lst : LinkedList<String> = ls::tail("/Users/rasrivastava/a.txt",3);
//    for line in lst.iter() {
//        println!("{}",line);
//    }
      //ls::find(".","".to_string());
    let index_count = ls::create_search_index("/Users/rasrivastava/tmp.txt","/Users/rasrivastava/index.txt");
    ls::binary_search_file("/Users/rasrivastava/index.txt","/Users/rasrivastava/tmp.txt",25,"\"slip\"",index_count);



}
