// 1233. Remove Sub-Folders from the Filesystem
// --------------------------------------------
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    end: bool,
    nodes: HashMap<String, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            end: false,
            nodes: HashMap::new(),
        }
    }

    pub fn add(&mut self, path: &Vec<String>, position: usize) {
        let key = path[position].clone();
        if self.nodes.contains_key(&key) {
            let node = self.nodes.get_mut(&key).unwrap();
            if position == path.len() - 1 {
                node.end = true;
            } else {
                node.add(path, position + 1);
            }
        } else {
            let mut node = Trie::new();
            if position == path.len() - 1 {
                node.end = true;
            } else {
                node.add(path, position + 1);
            }
            self.nodes.insert(key, node);
        };
    }

    pub fn contains(&self, path: &Vec<String>, position: usize) -> bool {
        if position == path.len() {
            return self.end;
        }
        let key = path[position].clone();
        if !self.nodes.contains_key(&key) {
            return false;
        }
        let node = self.nodes.get(&key).unwrap();
        if node.end {
            return true;
        }
        node.contains(path, position + 1)
    }
}

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        folder.iter().for_each(|path| {
            let parts = path
                .split('/')
                .map(|p| p.to_string())
                .filter(|p| p != "")
                .collect::<Vec<_>>();
            trie.add(&parts, 0);
        });
        let mut ans = vec![];

        folder.iter().for_each(|path| {
            let parts = path
                .split('/')
                .map(|p| p.to_string())
                .filter(|p| p != "")
                .collect::<Vec<_>>();
            let n = parts.len();
            if n == 1 {
                ans.push(path.clone());
            } else {
                let prefix = &parts[0..n-1].to_vec();
                if !trie.contains(&prefix, 0) {
                    ans.push(path.clone());
                }
            }
        });

        ans
    }
}
