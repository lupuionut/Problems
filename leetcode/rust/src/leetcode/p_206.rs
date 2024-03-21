// 206. Reverse Linked List
// ------------------------

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn walk(
            node: &Option<Box<ListNode>>,
            previous: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if node.is_none() {
                return previous;
            } else {
                let n = node.as_ref().unwrap();
                let mut new_node = ListNode::new(n.val);
                new_node.next = previous;
                return walk(&n.next, Some(Box::new(new_node)));
            }
        }

        walk(&head, None)
    }
}
