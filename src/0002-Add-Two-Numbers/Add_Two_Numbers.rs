// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::merge(l1, l2, 0, ListNode::new(-1))
    }

    fn merge (mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>, mut val: i32, mut ln: ListNode) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && val == 0 {
            return None;
        }

        if let Some(n1) = l1 {
            val += n1.val;
            l1 = n1.next;
        }
        if let Some(n2) = l2 {
            val += n2.val;
            l2 = l2.next;
        }

        ln.val = if val > 9 {val - 10} else { val };
        ln.next = Solution::merge(l1, l2, val / 10, ListNode::new(-1));
        Some(Box::new(ln))
    }
}