struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::remove_duplicates(&mut vec![1,1,2]));
    cases.push(Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 1 { return 1; }
        nums.dedup();
        let diff = nums.len();
        nums.resize(len, 0);

        diff as i32
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)