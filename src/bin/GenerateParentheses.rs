struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::generate_parenthesis(8));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        Self::add_parenthesis(0, 0, n, &mut String::from(""), &mut result);
        result
    }

    fn add_parenthesis(open: i32, closed: i32, n: i32, s: &mut String, r: &mut Vec<String>) {
        if s.len() as i32 == n*2 {
           r.push(s.to_string());
            return;
        }

        if open < n {
            s.push('(');
            Self::add_parenthesis(open + 1, closed, n, s, r);
            s.pop();
        }

        if closed < open {
            s.push(')');
            Self::add_parenthesis(open, closed + 1, n, s, r);
            s.pop();
        }
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)