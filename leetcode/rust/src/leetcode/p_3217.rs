// 3217. Delete Nodes From Linked List Present in Array
// ----------------------------------------------------
use std::collections::HashSet;
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut banned = HashSet::new();
        for num in nums {
            banned.insert(num);
        }
        fn f(node: Option<Box<ListNode>>, banned: &HashSet<i32>) -> Option<Box<ListNode>> {
            if node.is_none() {
                return None;
            }
            let mut node = node.unwrap();
            if banned.contains(&node.val) {
                return f(node.next, banned);
            } else {
                node.next = f(node.next, banned);
            }
            Some(node)
        }
        f(head, &banned)
    }
}
