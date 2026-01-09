struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::plus_one(vec![9,8,9]));
    cases.push(Solution::plus_one(vec![8,9,9,9]));
    cases.push(Solution::plus_one(vec![1,2,3]));
    cases.push(Solution::plus_one(vec![4,3,2,1]));
    cases.push(Solution::plus_one(vec![9]));
    cases.push(Solution::plus_one(vec![9, 9]));
    cases.push(Solution::plus_one(vec![9,0, 9]));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let len = digits.len() -1;
        let mut remainder = false;
        let mut ret = vec![];

        digits[len] = digits[len] +1;

        for i in (0..=len).rev() {
            if remainder {
                digits[i] = digits[i] + 1;
                remainder = false;
            }

            if digits[i] == 10 {
                ret.insert(0, 0);
                remainder = true;
            } else {
                ret.insert(0, digits[i]);
            }
        }

        if digits[0] == 10 {
            ret[0] = 0;
            ret.insert(0, 1);
        }

        ret
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)