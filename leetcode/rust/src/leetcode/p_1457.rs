// 1457. Pseudo-Palindromic Paths in a Binary Tree
// -----------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut values = [0; 10];
        Solution::dfs(root, &mut ans, &mut values);
        ans
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, values: &mut [i32; 10]) {
        if node.is_none() {
            return;
        }
        let node = node.unwrap();
        let node = node.borrow();
        let key = node.val as usize;
        values[key] += 1;

        // is leaf
        if node.left.is_none() && node.right.is_none() {
            if Solution::is_pseudo_panlindrome(values) {
                *ans += 1;
            }
            values[key] -= 1;
            return;
        }

        Solution::dfs(node.left.clone(), ans, values);
        Solution::dfs(node.right.clone(), ans, values);
        values[key] -= 1;
    }

    fn is_pseudo_panlindrome(values: &mut [i32; 10]) -> bool {
        let mut par = 0;
        let mut impar = 0;
        let mut total = 0;
        values.iter().for_each(|&n| {
            if n & 1 == 1 {
                impar += 1;
            } else if n > 0 {
                par += 1;
            }
            total += n;
        });
        if total & 1 == 0 {
            if impar > 0 {
                return false;
            }
            return true;
        } else {
            if impar != 1 {
                return false;
            }
            return true;
        }
    }
}
