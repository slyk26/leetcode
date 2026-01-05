struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::roman_to_int(String::from("DCXXI")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut prev = s.chars().rev().next().unwrap();
        let mut sum = Self::lookup(prev);

        for i in s.chars().rev().skip(1) {
            let v = Self::lookup(i);
            if v >= Self::lookup(prev) {
                sum += v;
            } else {
                sum -=v;
            }
            prev = i;
        }
        sum
    }

    fn lookup(c :char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)