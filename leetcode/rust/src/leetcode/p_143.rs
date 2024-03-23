// 143. Reorder List
// -----------------

use std::collections::VecDeque;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut curr = head.as_mut().unwrap().next.take();
        let mut queue = VecDeque::new();

        while let Some(mut node) = curr {
            curr = node.next.take();
            queue.push_back(node);
        }

        let mut back = true;
        let mut curr = head.as_mut().unwrap();

        while !queue.is_empty() {
            let t = if back {
                queue.pop_back().unwrap()
            } else {
                queue.pop_front().unwrap()
            };
            curr.next = Some(t);
            curr = curr.next.as_mut().unwrap();
            back = !back;
        }
    }
}
