// 2095. Delete the Middle Node of a Linked List
// ---------------------------------------------
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n = 0;
        let mut node = &head;
        while node.is_some() {
            n += 1;
            node = &node.as_ref().unwrap().next;
        }
        fn build(i: i32, node: &Option<Box<ListNode>>) -> Option<ListNode> {
            let mut n = node;
            if i == 0 {
                n = &node.as_ref().unwrap().next;
            }
            if n.is_none() {
                return None;
            }
            let mut t = ListNode::new(n.as_ref().unwrap().val);
            let nxt = build(i - 1, &n.as_ref().unwrap().next);
            if let Some(v) = nxt {
                t.next = Some(Box::new(v));
            } 
            return Some(t);
        }

        let mut i = n / 2;
        let mut dummy = ListNode::new(-1);
        let mut node = &head;
        let nxt = build(i, node);
        if let Some(v) = nxt {
            dummy.next = Some(Box::new(v));
        }

        dummy.next
    }
}
