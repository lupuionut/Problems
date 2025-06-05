// 1061. Lexicographically Smallest Equivalent String
// --------------------------------------------------

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut ans = vec![];
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = base_str.chars().collect();

        let mut parent = vec![0; 26];
        for i in 0..26 {
            parent[i] = i;
        }

        fn union(c1: usize, c2: usize, parent: &mut Vec<usize>) {
            let parent_c1 = find(c1, parent);
            let parent_c2 = find(c2, parent);
            if parent_c1 != parent_c2 {
                if parent_c1 < parent_c2 {
                    parent[parent_c2] = parent_c1;
                } else {
                    parent[parent_c1] = parent_c2;
                }
            }
        }

        fn find(c1: usize, parent: &mut Vec<usize>) -> usize {
            if parent[c1] != c1 {
                parent[c1] = find(parent[c1], parent);
            }
            parent[c1]
        }

        for i in 0..s1.len() {
            let c1 = s1[i] as usize - 97;
            let c2 = s2[i] as usize - 97;
            union(c1, c2, &mut parent);
        }

        for i in 0..base_str.len() {
            let k = find(s3[i] as usize - 97, &mut parent);
            ans.push(char::from(97 + k as u8));
        }

        ans.iter().collect::<String>()
    }
}
