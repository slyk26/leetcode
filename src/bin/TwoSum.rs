struct Solution;

fn main() {
    let mut cases = vec![];

    cases.push(Solution::two_sum(vec![2, 7, 11, 15], 9));
    cases.push(Solution::two_sum(vec![3, 2, 4], 6));
    cases.push(Solution::two_sum(vec![3, 3], 6));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let result : Vec<i32> = vec![];

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j && nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32]
                }
            }
        }
        result
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)