struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::my_atoi(String::from("-2147483647"))); // -2147483647
    cases.push(Solution::my_atoi(String::from("      -20000000000000000000000000x"))); // -2147483648
    cases.push(Solution::my_atoi(String::from("20000000000000000000"))); // 2147483647
    //cases.push(Solution::my_atoi(String::from("21474836460"))); // 2147483647
    //cases.push(Solution::my_atoi(String::from("1")));
    // cases.push(Solution::my_atoi(String::from("-+12")));
    //  cases.push(Solution::my_atoi(String::from("+-12")));
    // cases.push(Solution::my_atoi(String::from("+1")));
    // cases.push(Solution::my_atoi(String::from("-")));
    // cases.push(Solution::my_atoi(String::from("")));
    // cases.push(Solution::my_atoi(String::from("-91283472332"))); // -2147483648
    // cases.push(Solution::my_atoi(String::from("42")));
    // cases.push(Solution::my_atoi(String::from("-042")));
    // cases.push(Solution::my_atoi(String::from("1337c0d3")));
    // cases.push(Solution::my_atoi(String::from("0-1")));
    // cases.push(Solution::my_atoi(String::from("words and 987")));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use std::num::IntErrorKind::PosOverflow;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let b = s.as_bytes();
        let mut idx = 0;
        let mut end = 0;
        let mut num = 0;
        let mut cur_step = 1;
        let mut neg = false;
        let mut eol = false;

        if s.trim().is_empty() { return num; }

        while cur_step != 4 {
            match cur_step {
                1 => {
                    if b[idx].is_ascii_whitespace() { idx +=1; } else { cur_step = 2}
                },
                2 => {
                    if b[idx] == b'-' {
                        neg = true;
                        idx += 1;
                    } else if b[idx] == b'+'  { idx += 1; }

                    cur_step = 3;
                    end = idx;
                },
                3 => {
                    if end == s.len() { return num; }
                    if !b[end].is_ascii_digit() || eol  {
                        if eol { end +=1 }
                        
                        let r = match &s[idx..end].parse::<i64>() {
                            Ok(n) => *n ,
                            Err(err) => {
                                if err.kind() == &PosOverflow {  i32::MAX as i64 +1 } else { 0 }
                            }
                        };


                        if r > i32::MAX as i64 {
                            if neg {
                                num = - i32::MAX as i64 as i32 -1;
                            } else {
                                num = (i32::MAX as i64) as i32;
                            }
                        } else {
                            if neg {
                                num = -r as i32;
                            } else {
                                num = r as i32;
                            }
                        }

                        cur_step = 4;
                    }

                    else  {
                        if end == s.len()-1 {
                            eol = true;
                        } else {
                            end += 1;
                        }
                    }
                },
                _  => {panic!("lol")}
            }
        }
        num
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)