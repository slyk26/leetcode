struct Solution;


fn main() {
    let mut cases = vec![];

    let v = &mut vec![-1,0,0,3,3,3,0,0,0];
    let v0 = &mut vec![4,5,6,0,0,0];
    let v1 = &mut vec![1,2,3,0,0,0];
    let v4 = &mut vec![0];
    let v5 = &mut vec![-1,0,0,0,3,0,0,0,0,0,0];
    let v6 = &mut vec![1,0];

    Solution::merge(v0, 3, &mut vec![1,2,3,], 3);
    Solution::merge(v, 6, &mut vec![1,2,2], 3);
    Solution::merge(v1, 3, &mut vec![2,5,6], 3);
     Solution::merge(v4, 0, &mut vec![1], 1); // 1
     Solution::merge(v5, 5, &mut vec![-1,-1,0,0,1,2], 6); // [-1,-1,-1,0,0,0,0,0,1,2,3]
     Solution::merge(v6, 2, &mut vec![2], 1); // [-1,-1,-1,0,0,0,0,0,1,2,3]

    cases.push(v0);
    cases.push(v1);
    cases.push(v);
     cases.push(v4);
      cases.push(v5);
      cases.push(v6);

    println!("{:?}", cases)
}

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, _: i32, nums2: &mut Vec<i32>, _: i32) {
        for (n1,n2) in nums1.iter_mut().rev().zip(nums2.iter()) {
            *n1 = *n2;
        }
        nums1.sort();
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)