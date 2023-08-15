// 86. Partition List
// ------------------

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left = ListNode::new(0);
        let mut right = ListNode::new(0);
        let mut dummy_left = &mut left;
        let mut dummy_right = &mut right;

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                dummy_left.next = Some(node);
                dummy_left = dummy_left.next.as_mut().unwrap();
            } else {
                dummy_right.next = Some(node);
                dummy_right = dummy_right.next.as_mut().unwrap();
            }
        }
        dummy_left.next = right.next.take();

        left.next
    }
}
