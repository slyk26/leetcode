struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::is_palindrome(String::from("aa")));
    cases.push(Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")));
    cases.push(Solution::is_palindrome(String::from("race a car")));
    cases.push(Solution::is_palindrome(String::from(" ")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut i = 0;
        let mut j = s.len() -1;
        let b = s.as_bytes();

        while i < j {
            if !b[i].is_ascii_alphanumeric() {
                i +=1;
                continue;
            }

            if !b[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }

            if b[i].to_ascii_uppercase() != b[j].to_ascii_uppercase() {
                return false;
            } else {
                i += 1;
                j -= 1;
            }
        }
        true
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)