struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::number_of_steps(14));
    cases.push(Solution::number_of_steps(8));
    cases.push(Solution::number_of_steps(123));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut steps = 0;
        while num > 0 {
            if num & 1 == 0 {
                num >>=1;
            } else {
                num -= 1;
            }
            steps += 1;
        }
        steps
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)