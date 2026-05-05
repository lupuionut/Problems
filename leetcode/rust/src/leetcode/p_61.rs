// 61. Rotate List
// ---------------
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut n = 0;
        let mut node = &head;
        while node.is_some() {
            n += 1;
            node = &node.as_ref().unwrap().next;
        }
        let mut k = k % n;
        let mut t = n - k;

        let mut second = ListNode::new(-1);
        let mut node = &mut second;
        let mut original = &head;
        let mut first = ListNode::new(-1);

        while t >= 0 {
            if t == 0 {
                first.next = node.next.clone();
                node.next = None;
                break;
            }
            node.next = original.clone();
            original = &original.as_ref().unwrap().next;
            node = node.next.as_mut().unwrap();
            t -= 1;
        }

        let mut node = &mut first;
        while node.next.is_some() {
            let next = node.next.as_mut();
            node = node.next.as_mut().unwrap();
        }
        node.next = second.next;
        first.next
    }
}
