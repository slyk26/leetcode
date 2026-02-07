struct Solution;

fn main() {
    let mut cases = vec![];

    // cases.push(Solution::contains_nearby_duplicate(vec![1,2,3,1], 3));
    cases.push(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    // cases.push(Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2));
    println!("{:?}", cases)
}

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut m = HashMap::<i32, usize>::new();

        for i in 0..nums.len() {
            if let Some(idx) = m.get(&nums[i]) {
                if idx.abs_diff(i) as i32 <= k {
                    return true;
                }
            }
            m.insert(nums[i], i);
        }
        false
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
