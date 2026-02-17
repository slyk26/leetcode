struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::divide(10, 3));
    cases.push(Solution::divide(7, -3));
    cases.push(Solution::divide(-2147483648, -1));

    println!("{:?}", cases)
}

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        dividend.saturating_div(divisor)
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)