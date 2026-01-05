struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::str_str(String::from("sadbutsad"), String::from("sad")));
    cases.push(Solution::str_str(String::from("leetcode"), String::from("leeto")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(f) = haystack.find(&needle) {
            f as i32
        } else {
            -1
        }
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)