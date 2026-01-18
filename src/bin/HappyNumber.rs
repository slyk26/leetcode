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
        let mut first = true;
        let mut y = n;
        while y != 1 {
            y = Self::calc(Self::arr(y));
            if y == 4 && !first { return false; }
            first = false;
        }
        true
    }

    fn arr(n: i32) -> Vec<i32> {
        n.to_string()
            .as_bytes()
            .iter()
            .map(|b| (b - 48) as i32)
            .collect::<Vec<i32>>()
    }

    fn calc(digits: Vec<i32>) -> i32 {
        digits.iter().fold(0, |acc, digit| acc + digit.pow(2))
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
