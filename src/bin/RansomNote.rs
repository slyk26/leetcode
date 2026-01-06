use std::collections::{HashMap, HashSet};

struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::can_construct(String::from("a"), String::from("b")));
    cases.push(Solution::can_construct(String::from("aa"), String::from("ab")));
    cases.push(Solution::can_construct(String::from("aa"), String::from("aab")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn can_construct(ransom_note: String, mut magazine: String) -> bool {
        for c in ransom_note.chars() {
            if let Some(i) = magazine.find(c) {
                magazine.remove(i);
            } else {
                return false;
            }
        }
        true
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)