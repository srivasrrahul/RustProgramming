pub fn sort_bubble(arr : & mut [i32]) -> (){
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] > arr[j] {
                let mut temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
}

pub fn merge_sort(arr : & mut [i32],len : usize) -> usize {
    //merge sorted list first and second
    //begin to mid
    //mid+1 to end
    fn merge(local_arr : &mut[i32],begin : usize,mid : usize,end: usize) -> usize {
        //println!("Begin {} End {}",begin,end);
        let mut count_inversion: usize = 0;
        let mut p_first: usize = begin;
        let mut p_second : usize = mid+1;
        let mut vec = Vec::new();
        while p_first <= mid && p_second <= end {
            if local_arr[p_first] <= local_arr[p_second] {
                vec.push(local_arr[p_first]);
                p_first += 1;
            }else {
                count_inversion += (mid-p_first+1);
                vec.push(local_arr[p_second]);
                p_second +=1 ;
            }
        }

        while p_first <= mid {
            count_inversion += (end-(mid+1)+1);

            vec.push(local_arr[p_first]);
            p_first += 1;
        }

        while p_second <= end {
            vec.push(local_arr[p_second]);
            p_second +=1 ;
        }

        let mut index : usize = 0;
        let mut loop_index = begin;
        //println!("{}",vec.len());
        while loop_index <= end {
            local_arr[loop_index] = vec[index];
            loop_index += 1;
            index += 1;

        }

        return count_inversion;
//        let bar = || {
//            for i in begin..end + 1 {
//                arr[i] = vec[index];
//                index += 1;
//            }
//        };
//        bar();


    }

    fn merge_sort_rec(local_arr : &mut[i32],begin : usize,end : usize) -> usize {
        //println!("Hello");
        if begin >= end {
            return 0;
        } else {
            let mid_point: usize = begin  +  (end - begin) / 2;
            let left = merge_sort_rec(local_arr, begin, mid_point);
            let right = merge_sort_rec(local_arr, mid_point + 1, end);
            let post_merge = merge(local_arr, begin, mid_point, end);
            return left+right+post_merge;
        }
    }

    return merge_sort_rec(arr ,0,len);
}



