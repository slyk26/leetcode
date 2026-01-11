struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::hamming_weight(11));
    cases.push(Solution::hamming_weight(128));
    cases.push(Solution::hamming_weight(2147483645));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        format!("{:b}", n).bytes().filter(|&x| x == b'1').count() as i32
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)