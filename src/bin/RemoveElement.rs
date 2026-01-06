struct Solution;
struct Solution2;

fn main() {
    let mut cases = vec![];

    //cases.push(Solution::remove_element(&mut vec![1], 1));
    cases.push(Solution::remove_element(&mut vec![3,3], 3));
    //cases.push(Solution::remove_element(&mut vec![2], 3));
    //cases.push(Solution::remove_element(&mut vec![3,2,2,3], 3));
    //cases.push(Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2));

    println!("{:?}", cases)
}

impl Solution2 {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        nums.retain(|&x| x != val);
        let diff = nums.len();
        nums.resize(len, 0);
        diff as i32
    }
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use std::mem;

impl Solution {
    // 0.04mb smaller than Solution2
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() { return 0; }
        let mut cnt = 0;
        let mut i = 0;
        let mut j = nums.len()-1;

        while i <= j {
            if nums[i] != val {
                cnt +=1;
                i +=1;
            } else {
                if nums[j] != val {
                    nums.swap(i, j);
                } else {
                    if i == j {
                        mem::swap(&mut nums[j], &mut -1);
                        return cnt;
                    }
                    j -=1;
                }
            }
        }
        cnt
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)