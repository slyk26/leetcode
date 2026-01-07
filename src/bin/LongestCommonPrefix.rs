struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("flight")]));
    cases.push(Solution::longest_common_prefix(vec![String::from("dog"), String::from("racecar"), String::from("car")]));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ret = String::from("");
        let mut i = 0;
        let x : Vec<&[u8]> = strs.iter().map(|s| s.as_bytes()).collect();
        while !x.iter().any(|str| str.len() == i) {
            let ch = x[0][i];
            if x.iter().any(|str| str[i] != ch) {
                return ret;
            } else {
                ret.push(ch.into());
            }
            i += 1;
        }
        ret
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)