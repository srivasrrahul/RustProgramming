use std::vec;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::LinkedList;
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::io::SeekFrom;
use std::io::LineWriter;

pub fn list_files(file_path : &str) -> Vec<String> {
    let mut vec = Vec::new();
    return vec;
}

pub fn list_lines(file_path : &str) -> i32 {
    //let mut vec : Vec<String> = Vec::new();
    let f = File::open(file_path).unwrap();
    let mut file = BufReader::new(&f);
    for line in file.lines() {
        println!("{}",line.unwrap());
    }
//    for line in BufReader::new(file).lines()  {
//        println!("{:?}",line);
//    }
    return 10;


}

pub fn tail(file_path : &str, n : usize) -> LinkedList<String> {
    let f = File::open(file_path).unwrap();
    let mut file = BufReader::new(&f);
    let mut lst : LinkedList<String> = LinkedList::new();
    for line in file.lines() {
        if lst.len() == n {
            lst.pop_front();
        }

        lst.push_back(line.unwrap());
    }

    return lst;
}

fn iterate(path : &Path) {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let p = path.unwrap().path();
        let mut is_dir = false;
        //println!("{}",p.display());
        if p.is_dir() {
            //println!("Dir {}",p.display())
            is_dir = true;
        }else {
            println!("{}",p.display());
        }

        if is_dir {
            iterate(&p);
        }
    }
}

pub fn find(file_path : &str,key : String) {
    let p = Path::new(file_path);
    iterate(p);
}


pub fn create_search_index(file_path : &str,index_file_path : &str) -> i32 {
    //let p = Path::new(file_path);
    let f = File::open(file_path).unwrap();
    let mut file = BufReader::new(&f);
    let mut byte_index = 0;

    let mut index_map = BTreeMap::new();
    for line in file.lines() {
        let current_index = byte_index;
        let s = line.unwrap();
        byte_index = byte_index + s.len() + 1; //1 for newline char
        let mut split = s.split(",");
        let vec : Vec<&str>= split.collect();
        let val = vec[0];
        let mut st = String::new();
        index_map.insert(String::from(val),current_index);
//        st.push_str(val);
//        st.push_str(",");
//        st.push_str(&current_index.to_string());
//        st.push_str("\n");
//        //write!(st,"{}{}\n",val,current_index);
//        //write!(write_file,"{},{}\n",val,current_index);
//        write_file.write_all(&st.as_bytes());
    }

    let mut write_file = File::create(index_file_path).unwrap();
    for (name,loc) in &index_map{
        let mut st = String::new();
        st.push_str(name);
        st.push_str(",");
        st.push_str(&loc.to_string());
        if st.len() < 25 {
            let pending_len = 25 - st.len();
            let pending_str = "|".repeat(pending_len);
            st.push_str(&pending_str);
        }else {
            println!("Bad assumption");
        }

        write_file.write_all(&st.as_bytes());
    }

    return index_map.len() as i32;



}

//pub fn load_index_memory(index_file_path : &str) -> HashMap<String,i32> {
//    //let p = Path::new(index_file_path);
//    let f = File::open(index_file_path).unwrap();
//    let file = BufReader::new(&f);
//    let mut index_map = HashMap::new();
//    for line in file.lines() {
//        let s = line.unwrap();
//        let mut split = s.split(",");
//        let vec : Vec<&str> = split.collect();
//        let val = vec[0];
//        let dest_str = vec[1];
//        let dest_int = dest_str.parse::<i32>();
//
//
//        index_map.insert(String::from(val),dest_int.unwrap());
//    }
//
//    return index_map;
//
//
//
//}

//pub fn read_file(file : &mut File,b_size : i32,index: i32)  {
//    let s : u64= index as u64 * b_size as u64;
//    file.seek(SeekFrom::Start(s));
//    let mut buffer = vec![0;b_size as usize];
//    file.read_exact(&mut buffer);
//
//}
pub fn binary_search_file(index_file_path : &str,data_file_path : &str,block_size : usize,key : &str,index_count : i32) {
    fn read_file_index(file : &mut File,b_size : i32,index: i32) -> i32 {
        let s : u64= index as u64 * b_size as u64;
        file.seek(SeekFrom::Start(s));
        let mut buffer = vec![0;b_size as usize];
        file.read_exact(&mut buffer);
        let st : String = String::from_utf8(buffer).unwrap();
        //println!("{}",st);

        let spl = st.split(",");
        let split_vec : Vec<&str> = spl.collect();
        let dest_index = split_vec[1].replace("|","");
        //println!("{}",dest_index);
        return dest_index.parse::<i32>().unwrap();

    }

    fn read_raw_data(file_offset : i32, data_file : &mut BufReader<File>) -> (String,String) {
        let mut buffer = Vec::<u8>::new();
        data_file.seek(SeekFrom::Start(file_offset as u64));
        data_file.read_until(b'\n',&mut buffer).expect("failed to read completely");
        let mut s = String::from_utf8(buffer).unwrap();
        //println!("{}",s);
        s.replace("\n","");
        let vec : Vec<&str> = s.split(",").collect();
        //return String::from_utf8(buffer).unwrap().trim();
        return (String::from(vec[0]),s.clone());
    }

    fn read_data(index_offset : i32, index_file : &mut File,data_file : &mut BufReader<File>,block_size : i32) -> (String,String) {
        let raw_data_offset = read_file_index(index_file,block_size,index_offset);
        return read_raw_data(raw_data_offset,data_file);
    }

    fn bin_search(first : i32,last : i32,block_size : i32,index_file :&mut File,data_reader : &mut BufReader<File>,key : &str) -> i32 {
        if last < first {
            return -1;
        }


        let mid = first + (last-first)/2;
        let val = read_data(mid,index_file,data_reader,block_size).0;
        println!("Comparing {} and {}",val,key);
        if key == &val {
            return mid;
        }else {
            if key > &val {
                return bin_search(mid+1,last,block_size,index_file,data_reader,key);
            }else {
                return bin_search(first,mid-1,block_size,index_file,data_reader,key);
            }
            return 0;
        }

    }

    fn get_count_of_index(index_file_path : &str,block_size : u64) -> i32 {
        let metadata = fs::metadata(index_file_path).unwrap();
        let size = metadata.len();
        return size as i32/(block_size as i32);
    }
//    fn binary_search_internal(file : File,passed_key : &str,low : i32,high : i32,b_size : usize) {
//        let mid = low + (high-low)/2;
//
//    }
//    let mut f = File::open(index_file_path).unwrap();
//    let dest_file_path = read_file_index(&mut f,block_size as i32,101);
//    let mut data_file_buffer = BufReader::new(File::open(data_file_path).unwrap());
////    let mut buffer = Vec::<u8>::new();
////    data_file_buffer.seek(SeekFrom::Start(dest_file_path as u64));
////    data_file_buffer.read_until(b'\n',&mut buffer).expect("failed to read completelt");
////    println!("{}",String::from_utf8(buffer).unwrap());
//    let s1 : String = read_data(dest_file_path,&mut data_file_buffer) ;
//    let vec : Vec<&str> = s1.split(",").collect();
//    println!("{}",vec[0]);

    //let mut data_file_reader = BufReader::new(File::open(data_file_path).unwrap());
    //let index_count = get_count_of_index(index_file_path,block_size as u64);
    println!("{}",index_count);
    let mut data_file_reader = BufReader::new(File::open(data_file_path).unwrap());
    let mut index_file = File::open(index_file_path).unwrap();
    let index = bin_search(0 as i32,index_count-1 as i32,block_size as i32,&mut index_file,&mut data_file_reader,key);
    println!("{}",index);
    if index != -1 {
        let (k,v) = read_data(index,&mut index_file,&mut data_file_reader,block_size as i32);
        println!("{}",v);
    }

}

pub fn merge_sorted_files(file1 : &str,file2 : &str,merged_file : &str)  {
    let mut f1 = File::open(file1).unwrap();
    let mut f2 = File::open(file2).unwrap();

    let mut file_reader1 = BufReader::new(&f1);
    let mut file_reader2 = BufReader::new(&f2);

    let mut f3 = File::create(merged_file).unwrap();

    let mut file_line_writer = LineWriter::new(&f3);

    let mut f1_str = String::new();
    let mut res1 = file_reader1.read_line(&mut f1_str);

    let mut f2_str = String::new();
    let mut res2 = file_reader2.read_line(&mut f2_str);

    let l1 = f1_str.len();
    let l2 = f2_str.len();
//    f1_str.truncate(l1-1);
//    f2_str.truncate(l2-1);

    while true {
        match res1 {
            Ok(n1) => {
                match res2 {
                    Ok(n2) => {
                        if n1 > 0 && n2 > 0 {

                            //println!("f1 {}",f1_str);
                            //println!("f2 {}",f2_str);
                            if f1_str > f2_str {
                                //println!("inc f2");
                                file_line_writer.write_all(&mut f2_str.as_bytes());
                                //file_line_writer.write_all(b"\n");
                                f2_str.clear();
                                res2 = file_reader2.read_line(&mut f2_str);
//                                let l2 = f2_str.len();
//                                f2_str.truncate(l2-1);

                            } else {
                                if f2_str > f1_str {
                                    //println!("inc f1");
                                    file_line_writer.write_all(&mut f1_str.as_bytes());
                                    //file_line_writer.write_all(b"\n");
                                    f1_str.clear();
                                    res1 = file_reader1.read_line(&mut f1_str);
//                                    let l1 = f1_str.len();
//                                    f1_str.truncate(l1-1);
                                } else {
                                    file_line_writer.write_all(&mut f1_str.as_bytes());
                                    //file_line_writer.write_all(b"\n");
                                    f2_str.clear();
                                    f1_str.clear();
                                    res1 = file_reader1.read_line(&mut f1_str);
                                    res2 = file_reader2.read_line(&mut f2_str);
//                                    let l1 = f1_str.len();
//                                    let l2 = f2_str.len();
//                                    f1_str.truncate(l1-1);
//                                    f2_str.truncate(l2-1);
                                }
                            }
                        }else {
                            if n1 > 0 {
                                file_line_writer.write_all(&mut f1_str.as_bytes());
                            }

                            if n2 > 0 {
                                file_line_writer.write_all(&mut f2_str.as_bytes());
                            }
                            break;
                        }
                    },
                    Err(_) => {
                        break;
                    }
                }
            },
            Err(_) => {
                break;
            }
        }
    }

//
    for line1 in file_reader1.lines() {
        file_line_writer.write_all(&mut line1.unwrap().as_bytes());
        //file_line_writer.write_all(b"\n");
    }

    for line2 in file_reader2.lines() {
        file_line_writer.write_all(&mut line2.unwrap().as_bytes());
        //file_line_writer.write_all(b"\n");
    }



}

