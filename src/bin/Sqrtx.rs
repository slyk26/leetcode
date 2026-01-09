struct Solution;


fn main() {
    let mut cases = vec![];

    // cases.push(Solution::my_sqrt(4));
    // cases.push(Solution::my_sqrt(8));
    cases.push(Solution::my_sqrt(2147395599));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 || x == 1 { return x; }

        let mut low = 1;
        let mut high = x/2;
        let mut mid;
        let mut maybe = -1;

        while low <= high {
            mid = (low + high) / 2;
            if (mid as i64) * (mid as i64) <= x as i64 {
                maybe = mid;
                low = mid +1;
            } else {
                high = mid - 1;
            }
        }
        maybe
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)