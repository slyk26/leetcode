use std::ops::Deref;

struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::middle_node(to_ll(vec![1,2,3,4,5])));
    cases.push(Solution::middle_node(to_ll(vec![1,2,3,4,5,6])));

    println!("{:?}", cases)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

fn to_ll(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &val in vec.iter() {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }
    head
}

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l = Self::len(head.clone()) /2;
        let mut ptr = &mut head;

        for _ in 0..l {
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        ptr.clone()
    }

    fn len(mut head: Option<Box<ListNode>>) -> u32 {
        let mut ptr = &mut head;
        let mut l = 0;
        while ptr.is_some() {
            l += 1;
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        l
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)