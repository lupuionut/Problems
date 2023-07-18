// 234. Palindrome Linked List
// ---------------------------

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut previous: Option<Box<ListNode>> = None;
            let mut head = head;
            while head.is_some() {
                let mut t = ListNode::new(head.as_ref().unwrap().val);
                t.next = previous;
                previous = Some(Box::new(t));
                head = head.unwrap().next;
            }
            previous
        }

        let mut reversed = reverse(head.clone());
        let mut head = head;

        while head.is_some() {
            if head.as_ref().unwrap().val != reversed.as_ref().unwrap().val {
                return false;
            }
            head = head.unwrap().next;
            reversed = reversed.unwrap().next;
        }

        true
    }
}
