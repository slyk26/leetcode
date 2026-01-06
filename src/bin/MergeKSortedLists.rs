struct Solution;


fn main() {
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut vecs = lists.iter().map(|l| to_vec(l.clone())).flatten().collect::<Vec<_>>();
        vecs.sort_unstable();
        to_ll(vecs)
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)