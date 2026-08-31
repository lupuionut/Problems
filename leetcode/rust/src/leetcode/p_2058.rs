// 2058. Find the Minimum and Maximum Number of Nodes Between Critical Points
// --------------------------------------------------------------------------
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut curr_min: Option<(i32, i32)> = None;
        let mut curr_max: Option<(i32, i32)> = None;
        let mut prev = None;
        let mut critical = vec![];

        let mut curr = &head;
        let mut i = 0;
        while curr.is_some() {
            let node = curr.as_ref().unwrap();
            if let Some(v) = prev {
                if let Some(min) = curr_min {
                    if min.0 < node.val {
                        critical.push(min.1);
                    }
                }
                if let Some(max) = curr_max {
                    if max.0 > node.val {
                        critical.push(max.1);
                    }
                }
                if node.val > v {
                    curr_max = Some((node.val, i));
                    curr_min = None;
                } else if node.val < v {
                    curr_max = None;
                    curr_min = Some((node.val, i));
                } else {
                    curr_max = None;
                    curr_min = None;
                }
            } 
            prev = Some(node.val);
            curr = &node.next;
            i += 1;
        }

        if critical.len() < 2 {
            return vec![-1, -1];
        };

        let mut min = i32::MAX;
        let n = critical.len();
        for i in 1..n {
            min = min.min(critical[i] - critical[i-1]);
        }

        vec![min, critical[n-1]-critical[0]]
    }
}
