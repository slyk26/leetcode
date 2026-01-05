struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::merge_two_lists(to_ll(vec![1, 2, 4]), to_ll(vec![1, 3, 4])));
    cases.push(Solution::merge_two_lists(to_ll(vec![]), to_ll(vec![])));
    cases.push(Solution::merge_two_lists(to_ll(vec![]), to_ll(vec![0])));

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

//IMPORTANT!! Submit Code Region Begin(Do not remove this line)

fn to_ll(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &val in vec.iter() {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }
    head
}

fn to_vec(mut ll: Option<Box<ListNode>>) -> Vec<i32> {
    let mut ptr = &mut ll;
    let mut vec = vec![];

    while let Some(node) = ptr {
        vec.push(node.val);
        ptr = &mut node.next;
    }
    vec
}
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v1 = to_vec(list1);
        let v2 = &mut to_vec(list2);
        v1.append(v2);
        v1.sort();
        to_ll(v1)
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)