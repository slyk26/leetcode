struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::convert("PAYPALISHIRING".to_string(), 3));
    cases.push(Solution::convert("PAYPALISHIRING".to_string(), 4));
    cases.push(Solution::convert("A".to_string(), 1));
    cases.push(Solution::convert("AA".to_string(), 2));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 { return s }
        let mut str : Vec<char> = Vec::new();
        let x = Self::split(s, num_rows);

        for i in 0..num_rows as usize {
            for r in &x {
                str.push(*r.iter().nth(i).unwrap());
            }
        }

        str.into_iter().filter(|c| c.ne(&' ')).collect()
    }

    fn split(s: String, num_rows: i32) -> Vec<Vec<char>> {
        let mut j = 1;
        let mut r = vec![];
        let mut i = 0;
        while i < s.len() as i32 {
            if i % (num_rows + (num_rows - 2)) == 0 {
                j = 1;
                r.push(Self::fill_vec(i, &s, num_rows));
                i += num_rows;
            } else {
                r.push(Self::diagonal(i, j, &s, num_rows));
                j += 1;
                i += 1;
            }
        }
        r
    }

    fn fill_vec(offset: i32, s: &String, num_rows: i32) -> Vec<char> {
        let mut v = vec![];
        for i in 0..num_rows {
            v.push(s.chars().nth(offset as usize + i as usize).unwrap_or(' '));
        }
        v
    }

    fn diagonal(n: i32, j: i32, s: &String, num_rows: i32) -> Vec<char> {
        let mut v = vec![];
        for i in 0..num_rows {
            if i != j {
                v.push(' ');
            } else {
                v.push(s.chars().nth(n as usize).unwrap());
            }
        }
        v.reverse();
        v
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)