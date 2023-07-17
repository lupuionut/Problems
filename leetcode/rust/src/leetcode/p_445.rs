// 445. Add Two Numbers II
// -----------------------

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn reverse(l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut new_head = None;
            let mut l = l;

            while l.is_some() {
                let mut temp = ListNode::new(l.as_ref().unwrap().val);
                temp.next = new_head;
                new_head = Some(Box::new(temp));
                l = l.unwrap().next;
            }

            new_head
        }

        let mut l1 = reverse(l1);
        let mut l2 = reverse(l2);
        let mut temp = None;
        let mut carry = 0;
        let mut rest = None;

        while l1.is_some() && l2.is_some() {
            let mut val = l1.as_ref().unwrap().val + l2.as_ref().unwrap().val + carry;
            if val >= 10 {
                carry = 1;
                val = val % 10;
            } else {
                carry = 0;
            }
            let mut t = ListNode::new(val);
            t.next = temp;
            temp = Some(Box::new(t));
            l1 = l1.unwrap().next;
            l2 = l2.unwrap().next;
        }

        rest = if l1.is_some() { l1 } else { l2 };

        while rest.is_some() {
            let mut val = rest.as_ref().unwrap().val + carry;
            if val >= 10 {
                carry = 1;
                val = val % 10;
            } else {
                carry = 0;
            }
            let mut t = ListNode::new(val);
            t.next = temp;
            temp = Some(Box::new(t));
            rest = rest.unwrap().next;
        }

        if carry != 0 {
            let mut t = ListNode::new(1);
            t.next = temp;
            temp = Some(Box::new(t));
        }

        temp
    }
}
