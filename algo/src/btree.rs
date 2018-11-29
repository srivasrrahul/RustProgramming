//represents btree of size 12 of string size 25
const MAX_KEY_COUNT : usize = 7;
const MAX_RRN_COUNT : usize = 8;
pub struct BTreePage {
    values : [String;MAX_KEY_COUNT],
    keyCount : i32,
    rrns : [i32;MAX_RRN_COUNT],
    isLeaf : bool,
    location : i32
}

impl BTreePage {
    fn new() -> BTreePage {
        return BTreePage  { values : Default::default(),keyCount : 0,rrns : [0; MAX_RRN_COUNT],isLeaf : false,location : 0};
    }

    fn prior_rrn(record_index : i32) -> i32 {
        return record_index;
    }

    fn after_rrn(record_index : i32) -> i32 {
        return record_index+1;
    }

    fn mid_record_index() -> i32 {
        return (MAX_KEY_COUNT/2) as i32;
    }

    fn load_page(rrn : i32) -> BTreePage {
        //Need to load from disk as of now dummy implementation
        let mut page = BTreePage::new();
        page.values[0] = "P".to_string();
        page.values[1] = "Q".to_string();
        page.values[2] = "R".to_string();
        page.values[3] = "S".to_string();
        page.values[4] = "T".to_string();
        page.values[5] = "U".to_string();
        page.values[6] = "V".to_string();

        page.rrns[0] = 1;
        page.rrns[1] = 2;
        page.rrns[2] = 3;
        page.rrns[3] = 4;
        page.rrns[4] = 5;
        page.rrns[5] = 6;
        page.rrns[6] = 7;
        page.rrns[7] = 8;
        //return BTreePage::new();
        return page;
    }

    fn write_page(page : &BTreePage) -> i32 {
        return 10000;
    }

//    fn search(rrn : i32,key : &str) -> String {
//        let page = BTreePage::load_page(rrn);
//        let mut index = 0;
//        let mut found = false;
//        for val in page.values.iter() {
//            index = index + 1;
//            if val != "" {
//                if val == key {
//                    found = true;
//                    break;
//                } else {
//                    if key < val {
//                        break;
//                    }
//                }
//            }
//        }
//
//        if found {
//            let value = &page.values[index];
//            return value.to_string();
//            //return String::from(value);
//        }else {
//
//            return BTreePage::search(page.rrns[index],key);
//        }
//
//
//    }

//    fn insert(root_rrn : i32,inserted_data : &str) {
//        let page = BTreePage::load_page(root_rrn);
//        if page.keyCount == values.len()  {
//            //tree is full
//            let new_page = BTreePage::new();
//            new_page.rrns[0] = root_rrn;
//            split_child(new_page,0);
//        }
//    }

    fn tree_insert(root : &mut BTreePage,key : String) {
        if root.keyCount == MAX_KEY_COUNT as i32 {
            let mut s = BTreePage::new();
            s.isLeaf = false;
            s.rrns[0] = root.location;
            BTreePage::split_child(&mut s,root,0);

        }
    }
    fn split_child(page : &mut BTreePage,y : &mut BTreePage,record_index : i32) -> BTreePage {
        let mut new_page = BTreePage::new();
        //let mut y = BTreePage::load_page(page.rrns[record_index as usize]);
        new_page.isLeaf = y.isLeaf;
        new_page.keyCount = (MAX_KEY_COUNT/2) as i32; //get it from param
        let mid = BTreePage::mid_record_index();
        let mid_next  = BTreePage::mid_record_index() + 1;
        let mid_value = y.values[mid as usize].to_string();
        for j in 0..(new_page.keyCount) {
            new_page.values[j as usize] = y.values[(j+mid_next) as usize].to_string();
            y.values[(j+mid_next) as usize] = "".to_string(); //empty the string
            new_page.rrns[j as usize] = y.rrns[(j+mid_next) as usize];
            y.rrns[(j+mid_next) as usize] = 0; //remove the
            y.keyCount = y.keyCount-1; //reduce one

        }

        //copy last index
        new_page.rrns[new_page.keyCount as usize] = y.rrns[(MAX_RRN_COUNT-1)as usize];
        y.rrns[(MAX_RRN_COUNT-1)as usize] = 0;


//        if y.isLeaf == false {
//            for j in 0..12 {
//                new_page.rrns[j] = y.rrns[j + new_page.keyCount];
//                y.rrns[j + new_page.keyCount] = 0; //nullify old one
//            }
//        }

        println!("test {}, {}",record_index,(page.keyCount-1));
        for j in (record_index..(page.keyCount)).rev() {
            println!("j is {}",j);
            page.values[(j+1) as usize] = page.values[j as usize].to_string();
            page.rrns[BTreePage::after_rrn(j+1) as usize] = page.rrns[(j+1) as usize];
        }

        page.values[record_index as usize] = mid_value.to_string();
        let updated_page_location = BTreePage::write_page(&new_page);
        //new_page.location = updated_page_location;
        page.rrns[BTreePage::after_rrn(record_index) as usize] = updated_page_location;
        page.keyCount = page.keyCount+1;
        return new_page;

    }

    pub fn test_page_split() {
        let mut page = BTreePage::new();
        page.values[0] = "P".to_string();
        page.values[1] = "Q".to_string();
        page.values[2] = "R".to_string();
        page.values[3] = "S".to_string();
        page.values[4] = "T".to_string();
        page.values[5] = "U".to_string();
        page.values[6] = "V".to_string();

        page.rrns[0] = 1;
        page.rrns[1] = 2;
        page.rrns[2] = 3;
        page.rrns[3] = 4;
        page.rrns[4] = 5;
        page.rrns[5] = 6;
        page.rrns[6] = 7;
        page.rrns[7] = 8;
        page.keyCount = 6;

        let mut x = BTreePage::new();
        x.values[0] = "N".to_string();
        x.values[1] = "W".to_string();
        x.rrns[0] = 11;
        x.rrns[1] = 12;
        x.rrns[2] = 13;
        x.keyCount = 2;

        let new_page = BTreePage::split_child(&mut x, &mut page,1);
        println!("z page 0 {}",new_page.values[0]);
        println!("z page 1 {}",new_page.values[1]);
        println!("z page 2 {}",new_page.values[2]);
        println!("z page 3 {}",new_page.values[3]);

        println!("z page rrn 0 {}",new_page.rrns[0]);
        println!("z page rrn 1 {}",new_page.rrns[1]);
        println!("z page rrn 2 {}",new_page.rrns[2]);
        println!("z page rrn 2 {}",new_page.rrns[3]);
        println!("z page rrn 3 {}",new_page.rrns[4]);


        println!("y page 0 {}",page.values[0]);
        println!("y page 1 {}",page.values[1]);
        println!("y page 2 {}",page.values[2]);
        println!("y page 3 {}",page.values[3]);
        println!("y page 4 {}",page.values[4]);

        println!("x page 0 {}",x.values[0]);
        println!("x page 1 {}",x.values[1]);
        println!("x page 2 {}",x.values[2]);

        println!("x rrn 0 {}",x.rrns[0]);
        println!("x rrn 1 {}",x.rrns[1]);
        println!("x rrn 2 {}",x.rrns[2]);
        println!("x rrn 3 {}",x.rrns[3]);

        println!("x keycount {}",x.keyCount);
        println!("y keycount {}",page.keyCount);
        println!("z keycount {}",new_page.keyCount);
//        println!("x page 2 {}",x.values[7]);
//        println!("x page 3 {}",page.values[3]);
//        println!("x page 4 {}",page.values[4]);



    }


}