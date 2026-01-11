struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::contains_duplicate(vec![1,2,3,1]));
    cases.push(Solution::contains_duplicate(vec![1,2,3,4]));
    cases.push(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() != HashSet::<i32>::from_iter(nums.iter().cloned()).len()
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)