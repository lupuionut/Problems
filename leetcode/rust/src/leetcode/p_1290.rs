// 1290. Convert Binary Number in a Linked List to Integer
// -------------------------------------------------------

impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut ans = 0;
        while let Some(node) = head {
            ans = (ans << 1) + node.val;
            head = node.next;
        }
        ans
    }
}
