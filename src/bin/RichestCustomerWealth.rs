struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::maximum_wealth(vec![vec![1,2,3], vec![3,2,1]]));
    cases.push(Solution::maximum_wealth(vec![vec![1,5], vec![7,3], vec![3,5]]));
    cases.push(Solution::maximum_wealth(vec![vec![2,8,7], vec![7,1,3], vec![1,9,5]]));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|bank| bank.iter().sum()).max().unwrap()
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)