struct Solution;

fn main() {
    let mut cases = vec![];

    cases.push(Solution::reverse(123));
    cases.push(Solution::reverse(-123));
    cases.push(Solution::reverse(120));
    cases.push(Solution::reverse(1534236469));

    println!("{:?}", cases)
}

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut str = x.abs().to_string();
        let bytes = unsafe { str.as_bytes_mut() };
        let l = bytes.len();

        for i in 0..l/2 {
            bytes.swap(i, l-i-1);
        }

        let num = str.parse::<i32>().unwrap_or(0);
        if x < 0 { -num } else { num }
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
