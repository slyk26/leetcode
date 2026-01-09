struct Solution;


fn main() {
    let mut cases = vec![];

    cases.push(Solution::delete_duplicates(to_ll(vec![1,1,2])));
    //cases.push(Solution::delete_duplicates(to_ll(vec![1,1,2,3,3])));

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec = to_vec(head);
        vec.dedup();
        to_ll(vec)
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)