struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::search_insert(vec![1,3,5], 1));
    cases.push(Solution::search_insert(vec![1,3], 3));
    cases.push(Solution::search_insert(vec![1], 1));
    cases.push(Solution::search_insert(vec![1,3,5,6], 5));
    cases.push(Solution::search_insert(vec![1,3,5,6], 2));
    cases.push(Solution::search_insert(vec![1,3,5,6], 7));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() { return 0; }
        let mut l = 0;
        let mut r = nums.len() -1;
        let mut i = r >> 1;

        if target > nums[r] { return (r+1) as i32; }
        if target < nums[l] { return l as i32; }

        loop {
            if target == nums[i] {
                return i as i32;
            }

            if r-l == 1 {
                return (l+1) as i32;
            }

            // smaller
            if target < nums[i] {
                r = i;
                i = r >> 1;
            }

            // bigger
            if target > nums[i] {
                l = i;
                i = l + ((r-l)>>1)
            }

        }
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)