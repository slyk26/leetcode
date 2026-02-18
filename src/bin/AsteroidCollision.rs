struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::asteroid_collision(vec![5, 10, -5]));
    cases.push(Solution::asteroid_collision(vec![8, -8]));
    cases.push(Solution::asteroid_collision(vec![10, 2, -5]));
    cases.push(Solution::asteroid_collision(vec![3, 5, -6, 2, -1, 4]));
    // cases.push(Solution::asteroid_collision(vec![-2, -2, -2, -2]));
    // cases.push(Solution::asteroid_collision(vec![2, 2, 2, 2]));
    // cases.push(Solution::asteroid_collision(vec![-2, 1, -1, -1]));
    cases.push(Solution::asteroid_collision(vec![-2, 1, 2, -2]));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut r = vec![];

        for i in 0..asteroids.len() {
            if asteroids[i] > 0 {
                r.push(asteroids[i]);
            } else {
                if r.len() > 0 {
                    let mut e = r[r.len() - 1];
                    while e <= asteroids[i].abs() {
                        if e > 0 {
                            let v = r.pop().unwrap();

                            if v.abs() == asteroids[i].abs() {
                                break;
                            }
                        } else {
                            r.push(asteroids[i]);
                            break;
                        }

                        if r.len() > 0 {
                            e = r[r.len() - 1];
                        } else {
                            if e.abs() != asteroids[i].abs() {
                                r.push(asteroids[i]);
                            }
                            break;
                        }
                    }
                } else {
                    r.push(asteroids[i]);
                }
            }
        }
        r
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)