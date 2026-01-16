struct Solution;

fn main() {
    let mut cases = vec![];

    cases.push(Solution::length_of_longest_substring(String::from("jbpnbwwd"))); // 4
    cases.push(Solution::length_of_longest_substring(String::from("dvdf")));
    cases.push(Solution::length_of_longest_substring(String::from("aab")));
    cases.push(Solution::length_of_longest_substring(String::from("abcabcbb")));
    cases.push(Solution::length_of_longest_substring(String::from("bbbbb")));
    cases.push(Solution::length_of_longest_substring(String::from("pwwkew")));

    println!("{:?}", cases)
}

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() { return 0; };

        let b = s.as_bytes();
        let mut max : i32 = 0;
        let mut cur = 0;
        let mut j;

        for i in 0..s.len() {
            j = i;

            if max >= (b.len() - i) as i32 {
                return max;
            }

            while !b[i..j].contains(&b[j]) {
                cur += 1;
                j += 1;
                if j >= s.len() {
                    break
                }
            }

            if cur > max {
                max = cur;
            }
            cur = 0;
        }

        max
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)
