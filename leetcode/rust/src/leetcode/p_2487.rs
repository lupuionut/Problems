// 2487. Remove Nodes From Linked List
// -----------------------------------

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack = vec![];
        let mut node = head;
        let mut dummy = ListNode::new(-1);

        while node.is_some() {
            let n = node.unwrap();
            while stack.len() > 0 && *stack.last().unwrap() < n.val {
                stack.pop();
            }
            stack.push(n.val);
            node = n.next;
        }

        dummy.next = Self::attach(&stack, 0);
        dummy.next
    }

    fn attach(nodes: &Vec<i32>, i: usize) -> Option<Box<ListNode>> {
        if i == nodes.len() {
            return None;
        }

        let mut node = ListNode::new(nodes[i]);
        node.next = Self::attach(nodes, i + 1);
        Some(Box::new(node))
    }
}
