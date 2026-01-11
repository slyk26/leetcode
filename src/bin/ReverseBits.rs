struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::reverse_bits(43261596)); // 964176192
    cases.push(Solution::reverse_bits(2147483644)); // 1073741822

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        n.reverse_bits()
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)