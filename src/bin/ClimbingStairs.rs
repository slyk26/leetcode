
struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::climb_stairs(2));
    cases.push(Solution::climb_stairs(3));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
use std::collections::{HashMap};

impl Solution {

    pub fn climb_stairs(n: i32) -> i32 {
        Self::memo(n, &mut HashMap::new())
    }

    fn memo(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 { return 1; }
        if n <= 0 { return 0; }

        if cache.contains_key(&n) { return cache[&n]; }

        let total = Self::memo(n-1, cache) + Self::memo(n-2, cache);

        cache.insert(n, total);
        total
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)