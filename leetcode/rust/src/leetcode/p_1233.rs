// 1233. Remove Sub-Folders from the Filesystem
// --------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort();

        let mut seen = HashSet::new();
        let mut ans = Vec::new();

        folder.iter().for_each(|fd| {
            let paths = fd.split('/').collect::<Vec<_>>();
            let n = paths.len();

            if n == 2 {
                seen.insert((paths[1], 1));
                ans.push(fd.clone());
            }
            if n > 2 {
                let mut possible = true;
                for i in 1..n {
                    if seen.contains(&(paths[i], i)) {
                        possible = false;
                    }
                }
                if possible {
                    seen.insert((paths[n - 1], n - 1));
                    ans.push(fd.clone());
                }
            }
        });

        ans
    }
}
