struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::is_palindrome(1410110141));
    cases.push(Solution::is_palindrome(-121));
    cases.push(Solution::is_palindrome(10));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        if x < 10 { return true;}
        if x % 10 == 0 { return false; }
        let base: i32 = 10;

        for i in 1..Self::potency(x)/2+1{
            let last = (x%(base.pow(i)) - x%(base.pow(i - 1))) / base.pow(i-1);
            let mut first = x / base.pow(Self::potency(x)-i);

            if i > 1 { first = first  % base }

            if first != last {
                return false;
            }
        }

        true
    }

    fn potency(y: i32) -> u32 {
        let base: u64 = 10;
        let mut pot = 1;

        while base.pow(pot) < y as u64 {
            pot += 1;
        }
        pot
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)