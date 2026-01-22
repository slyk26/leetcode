struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::generate(5));
    cases.push(Solution::generate(1));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret : Vec<Vec<i32>> = vec![];

        for i in 1..=num_rows as usize {
            ret.push(match i {
                1 => vec![1],
                2 => vec![1, 1],
                _ => {
                    let mut calc : Vec<i32> = vec![];
                    for j in 0 ..i {
                        if j == 0 || j == i -1 {calc.push(1)} else {
                            calc.push(ret[i-2][j] + ret[i-2][j-1])
                        }
                    }
                    calc
                }
            })
        }

        ret
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)