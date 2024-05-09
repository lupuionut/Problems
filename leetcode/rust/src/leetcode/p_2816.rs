// 2816. Double a Number Represented as a Linked List
// --------------------------------------------------

impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (node, carry) = Solution::calculate(head);
        if carry != 0 {
            let mut new_node = ListNode::new(carry);
            new_node.next = node;
            return Some(Box::new(new_node));
        }
        node
    }

    fn calculate(node: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
        if node.is_none() {
            return (None, 0);
        }
        let node = node.unwrap();
        let (next, carry) = Self::calculate(node.next);
        let calculated = (2 * node.val + carry);
        let val = calculated % 10;
        let carry = calculated / 10;
        let mut node = ListNode::new(val);
        node.next = next;
        return (Some(Box::new(node)), carry);
    }
}
