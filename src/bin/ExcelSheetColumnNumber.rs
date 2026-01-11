struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::title_to_number(String::from("FXSHRXW")));
    cases.push(Solution::title_to_number(String::from("A")));
    cases.push(Solution::title_to_number(String::from("AB")));
    cases.push(Solution::title_to_number(String::from("ZY")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let s = column_title.as_bytes();
        let l = s.len();
        let mut sum = 0;

        for i in 0..l {
            sum += 26i32.pow(i as u32) * ((s[l-i-1] - 64) as i32)
        }

        sum
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)