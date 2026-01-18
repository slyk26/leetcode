struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::is_ugly(6));
    cases.push(Solution::is_ugly(1));
    cases.push(Solution::is_ugly(14));


    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {return false}

        while n%2 == 0 {
            n /= 2;
        }

        while n%3 == 0 {
            n /=3
        }

        while n%5 == 0 {
            n /=5
        }

        n == 1
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)