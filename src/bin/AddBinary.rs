struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::add_binary(String::from("100"), String::from("110010")));
    cases.push(Solution::add_binary(String::from("1111"), String::from("1111")));
     cases.push(Solution::add_binary(String::from("1"), String::from("111")));
    cases.push(Solution::add_binary(String::from("11"), String::from("1")));
    cases.push(Solution::add_binary(String::from("1010"), String::from("1011")));


    println!("{:?}", cases)
}

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut remainder = false;
        let mut s = String::new();
        let (a1, b1) = match a.len() >= b.len() {
            true => (a.as_bytes(), b.as_bytes()),
            false => (b.as_bytes(), a.as_bytes()),
        };

        for i in 0..a1.len() {
            match i < b1.len() {
                true => {
                    Self::add_bit(&mut s,  b1[b1.len() -1 -i] + a1[a1.len() -1 -i] - 96, &mut remainder, false);

                }
                false => {
                    Self::add_bit(&mut s,  a1[a1.len() -1 -i] & 1, &mut remainder, false);
                }
            }
        }

        if remainder {
            let first = s.remove(0) as u8;
            Self::add_bit(&mut s, first - 48, &mut remainder, true);
            s.insert(0, 49 as char);
        }
        s
    }

    fn add_bit(s: &mut String, mut bit: u8, remainder: &mut bool, last: bool) {
        if *remainder && !last {
            bit += 1;
        }

        *remainder = bit >=2;

        s.insert(0, ((bit & 1) + 48) as char);
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)