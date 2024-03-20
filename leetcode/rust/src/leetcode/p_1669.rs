// 1669. Merge In Between Linked Lists
// -----------------------------------

impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Solution::create_head(&list1, 0, a);
        let mut tail = Solution::create_tail(&list1, 0, b);
        let mut tail = Solution::create_list(&list2, tail);
        Solution::create_list(&head, tail)
    }

    fn create_list(
        node: &Option<Box<ListNode>>,
        tail: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if node.is_none() {
            return tail;
        }
        let node = node.as_ref().unwrap();
        let mut new_node = ListNode::new(node.val);
        new_node.next = Self::create_list(&node.next, tail);
        Some(Box::new(new_node))
    }

    fn create_head(node: &Option<Box<ListNode>>, step: i32, until: i32) -> Option<Box<ListNode>> {
        if node.is_none() {
            return None;
        }
        if step >= until {
            return None;
        } else {
            let node = node.as_ref().unwrap();
            let mut new_node = ListNode::new(node.val);
            new_node.next = Self::create_head(&node.next, step + 1, until);
            return Some(Box::new(new_node));
        }
    }

    fn create_tail(node: &Option<Box<ListNode>>, step: i32, from: i32) -> Option<Box<ListNode>> {
        if node.is_none() {
            return None;
        }
        if step > from {
            let node = node.as_ref().unwrap();
            let mut new_node = ListNode::new(node.val);
            new_node.next = Self::create_tail(&node.next, step + 1, from);
            return Some(Box::new(new_node));
        } else {
            let node = node.as_ref().unwrap();
            return Self::create_tail(&node.next, step + 1, from);
        }
    }
}
