struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::convert_to_title(52)); // AZ
    cases.push(Solution::convert_to_title(2147483647)); // FXSHRXW
    cases.push(Solution::convert_to_title(1));
    cases.push(Solution::convert_to_title(28));
    cases.push(Solution::convert_to_title(701));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut ret = String::new();

        let mut x = column_number;
        while x > 26 {
            let help = x % 26;

            match help {
                0 => {
                    ret.insert(0, 'Z');
                    x-=1;
                },
                _ => ret.insert(0, (help + 64) as u8 as char)
            }

            x /= 26;
        }

        ret.insert(0, (x + 64) as u8 as char);

        ret
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)