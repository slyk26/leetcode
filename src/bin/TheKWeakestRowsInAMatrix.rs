struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::k_weakest_rows(
        vec![
            vec![1,1,0,0,0],
            vec![1,1,1,1,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,0],
            vec![1,1,1,1,1]], 3
    ));
    cases.push(Solution::k_weakest_rows(
        vec![
            vec![1,0,0,0],
            vec![1,1,1,1],
            vec![1,0,0,0],
            vec![1,0,0,0]
        ], 2
    ));

    cases.push(Solution::k_weakest_rows(
        vec![
            vec![1,0],
            vec![1,0],
            vec![1,0],
            vec![1,1]
        ], 4
    ));

    println!("{:?}", cases)
}


//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut a = mat.iter().map(|v| v.iter().position(|&x| x == 0).unwrap_or(usize::MAX)).enumerate().collect::<Vec<_>>();
        a.sort_by_key(|&(_, found)| found);
        a.iter().take(k as usize).map(|k| k.0 as i32).collect()
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)