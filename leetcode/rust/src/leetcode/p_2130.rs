// 2130. Maximum Twin Sum of a Linked List

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut max_sum = 0;
        let mut head = head;
        let mut values = Vec::new();

        while head.is_some() {
            if let Some(node) = head {
                let val = node.val;
                values.push(val);
                head = node.next;
            }
        }
        let n = values.len();
        for i in (0..n / 2) {
            let s = values[i] + values[n - 1 - i];
            if s > max_sum {
                max_sum = s;
            }
        }
        max_sum
    }
}
