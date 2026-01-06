struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::fizz_buzz(3));
    cases.push(Solution::fizz_buzz(5));
    cases.push(Solution::fizz_buzz(15));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = vec![];
        for i in 1..n+1 {
            let mut r : Vec<String> = vec![];
            let fizz = i% 3 == 0;
            let buzz = i % 5 == 0;

            if fizz {
                r.push("Fizz".to_string());
            }
            if buzz {
                r.push("Buzz".to_string());
            }

            if !fizz && !buzz {
                r.push(i.to_string());
            }
            res.push(r.join(""))
        }
        res
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)