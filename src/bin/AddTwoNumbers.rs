struct Solution;


fn main() {
    let mut cases = vec![];

    //cases.push(Solution::add_two_numbers(to_ll(vec![2,4,3]), to_ll(vec![5,6,4])));
    //cases.push(Solution::add_two_numbers(to_ll(vec![0]), to_ll(vec![0])));
    cases.push(Solution::add_two_numbers(to_ll(vec![9,9,9,9,9,9,9]), to_ll(vec![9,9,9,9])));

    println!("{:?}", cases)
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
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut remainder = false;
        let mut a = l1.as_ref();
        let mut b = l2.as_ref();
        let mut c :Option<Box<ListNode>> = None;
        let mut c_ptr = &mut c;

        loop {
            if a.is_none() && b.is_none() {

                if remainder {
                    *c_ptr = Some(Box::new(ListNode::new(1)));
                }

                break;
            }

            let a_val = Self::val(a);
            let b_val = Self::val(b);

            let mut sum = a_val + b_val;

            if remainder {
                sum += 1;
                remainder = false;
            }

            if sum >= 10 {
                remainder = true;
                sum = sum%10;
            }

            *c_ptr = Some(Box::new(ListNode::new(sum)));
            c_ptr = &mut c_ptr.as_mut().unwrap().next;

            a = Self::next(a);
            b = Self::next(b);

        }

        c
    }

    fn next(node: Option<&Box<ListNode>>) -> Option<&Box<ListNode>> {
        if let Some(node) = node {
            node.next.as_ref()
        } else {
            None
        }
    }

    fn val(node: Option<&Box<ListNode>>) -> i32 {
        if let Some(n) = node {
            n.val
        } else {
            0
        }
    }
}
//IMPORTANT!! Submit Code Region End(Do not remove this line)