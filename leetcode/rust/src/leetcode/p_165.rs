// 165. Compare Version Numbers
// ----------------------------

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut parts1 = version1
            .split(".")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut parts2 = version2
            .split(".")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        while parts1.len() < parts2.len() {
            parts1.push(0);
        }

        while parts2.len() < parts1.len() {
            parts2.push(0);
        }

        for i in 0..parts1.len() {
            if parts1[i] < parts2[i] {
                return -1;
            }
            if parts1[i] > parts2[i] {
                return 1;
            }
        }
        0
    }
}
