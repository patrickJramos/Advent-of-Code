use std::ops::Div;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let sum = l1.as_ref().unwrap().val + l2.as_ref().unwrap().val;

    let digit = sum % 10;
    let mut carry = sum >= 10;

    let mut n = Box::new(ListNode::new(digit));

    let mut curr_n1 = l1.as_ref();
    let mut curr_n2 = l2.as_ref();

    let mut current = &mut n;
    loop {
        curr_n1 = curr_n1.and_then(|x| x.next.as_ref());
        curr_n2 = curr_n2.and_then(|x| x.next.as_ref());

        if curr_n1.is_none() && curr_n2.is_none() {
            if carry {
                current.next = Some(Box::new(ListNode::new(1)));
            }
            return Some(n);
        }

        let sum = curr_n1.map(|x| x.val).unwrap_or(0)
            + curr_n2.map(|x| x.val).unwrap_or(0)
            + carry.then_some(1).unwrap_or(0);

        let digit = sum % 10;
        carry = sum >= 10;
        let dig = Box::new(ListNode::new(digit));
        current.next = Some(dig);
        current = current.next.as_mut().unwrap();
    }
}
