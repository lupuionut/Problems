// 95. Unique Binary Search Trees II
// ---------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn generate(from: i32, to: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if from == to {
                return vec![Some(Rc::new(RefCell::new(TreeNode::new(from))))];
            }
            if from > to {
                return vec![None];
            }

            let mut ans = vec![];
            for n in from..=to {
                let left = generate(from, n - 1);
                let right = generate(n + 1, to);

                for l in &left {
                    for r in &right {
                        let mut t = TreeNode::new(n);
                        t.left = l.clone();
                        t.right = r.clone();
                        ans.push(Some(Rc::new(RefCell::new(t))));
                    }
                }
            }

            ans
        }

        generate(1, n)
    }
}
