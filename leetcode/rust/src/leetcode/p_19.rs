// 19. Remove Nth Node From End of List
// ------------------------------------

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut new_head = &head;
        let mut k = 0;
        while k < n {
            if let Some(node) = new_head {
                new_head = &node.next;
            }
            k += 1;
        }

        fn advance(
            head: &Option<Box<ListNode>>,
            new_head: &Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match (head, new_head) {
                (Some(n), Some(m)) => {
                    let mut node = ListNode::new(n.val);
                    node.next = advance(&n.next, &m.next);
                    Some(Box::new(node))
                }
                (Some(n), None) => n.next.clone(),
                _ => None,
            }
        }

        advance(&head, new_head)
    }
}
