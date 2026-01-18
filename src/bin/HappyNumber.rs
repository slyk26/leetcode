struct Solution;

fn main() {
    let mut cases = vec![];

    cases.push(Solution::is_happy(3)); // false
    cases.push(Solution::is_happy(7)); // true
    cases.push(Solution::is_happy(4)); // false
    cases.push(Solution::is_happy(19)); // true
    cases.push(Solution::is_happy(2)); // false

    println!("{:?}", cases)
}

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut y = n;
        while y != 1 {
            y = Self::calc(y);
            if y == 4 { return false; }
        }
        true
    }

    fn calc(n: i32) -> i32 {
        n.to_string()
            .as_bytes()
            .iter()
            .fold(0, |acc, b| acc + (b - 48).pow(2) as i32)
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
