struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::is_valid(String::from("()")));
    cases.push(Solution::is_valid(String::from("()[]{}")));
    cases.push(Solution::is_valid(String::from("(]")));
    cases.push(Solution::is_valid(String::from("([])")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        for i in s.chars() {
            match i {
                '(' | '[' | '{' => stack.push(i),
                ')' | ']' | '}' => {
                    if let Some(c) =stack.pop() {
                        if c != Self::pair(i) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => panic!("lol")
            }
        }
        stack.is_empty()
    }

    fn pair(c: char) -> char {
        match c {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            _ => panic!("lol")
        }
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)