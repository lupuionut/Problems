// 876. Middle of the Linked List
// ------------------------------

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = head.as_ref();
        let mut len = 0;
        while node.is_some() {
            len += 1;
            node = node.unwrap().next.as_ref();
        }
        len /= 2;
        let mut node = head;
        while len > 0 {
            node = node.unwrap().next;
            len -= 1;
        }
        node
    }
}
