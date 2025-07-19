// 1233. Remove Sub-Folders from the Filesystem
// --------------------------------------------

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        let mut curr = " ".to_string();
        let mut ans = vec![];
        folder.sort();

        folder.into_iter().for_each(|f| {
            if !f.starts_with(&(curr.clone() + "/")) {
                ans.push(f.clone());
                curr = f;
            }
        });

        ans
    }
}
