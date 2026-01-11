struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::is_isomorphic(String::from("badc"), String::from("baba")));
    cases.push(Solution::is_isomorphic(String::from("egg"), String::from("add")));
    cases.push(Solution::is_isomorphic(String::from("foo"), String::from("bar")));
    cases.push(Solution::is_isomorphic(String::from("paper"), String::from("title")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut hm = HashMap::<u8, u8>::new();
        let mut mapped = HashSet::<u8>::new();
        let sb = s.as_bytes();
        let tb = t.as_bytes();

        for i in 0..sb.len() {
           if let Some( c) =hm.insert(sb[i], tb[i]) {
               if c != tb[i] {
                   return false;
               }
           } else {
               if mapped.contains(&tb[i]) {
                   return false;
               }
           }
            mapped.insert(tb[i]);
        }
        true
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)