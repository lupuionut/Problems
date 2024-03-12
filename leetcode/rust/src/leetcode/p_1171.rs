// 1171. Remove Zero Sum Consecutive Nodes from Linked List
// --------------------------------------------------------

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut stack: Vec<i32> = vec![];

        while curr.is_some() {
            let node = curr.unwrap();
            let mut is_found = false;

            if node.val == 0 {
                is_found = true;
            }

            if stack.len() > 0 {
                let needle = -node.val;
                let mut i = stack.len() as i32 - 1;
                let mut found = None;
                while i >= 0 {
                    if let Some(val) = found {
                        found = Some(val + stack[i as usize]);
                    } else {
                        found = Some(stack[i as usize]);
                    }

                    if let Some(val) = found {
                        if val == needle {
                            is_found = true;
                            stack.truncate(i as usize);
                        }
                    }
                    i -= 1;
                }
            }

            if !is_found {
                stack.push(node.val);
            }

            curr = node.next;
        }

        Solution::createList(&stack, 0)
    }

    fn createList(values: &Vec<i32>, i: usize) -> Option<Box<ListNode>> {
        if i == values.len() {
            return None;
        }
        let mut node = ListNode::new(values[i]);
        node.next = Self::createList(values, i + 1);
        Some(Box::new(node))
    }
}
