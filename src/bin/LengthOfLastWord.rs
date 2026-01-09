struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::length_of_last_word(String::from("a")));
    cases.push(Solution::length_of_last_word(String::from(" a")));
    cases.push(Solution::length_of_last_word(String::from("Hello World")));
    cases.push(Solution::length_of_last_word(String::from("   fly me   to   the moon  ")));
    cases.push(Solution::length_of_last_word(String::from("luffy is still joyboy")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().as_bytes().rsplit(|c| *c == 32).next().unwrap().len() as i32
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)