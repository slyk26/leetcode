struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::single_number(vec![2,2,1]));
    cases.push(Solution::single_number(vec![4,1,2,1,2]));
    cases.push(Solution::single_number(vec![1]));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut num = nums[0];

        for i in 1..nums.len() {
            num ^= nums[i];
        }

        num
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)