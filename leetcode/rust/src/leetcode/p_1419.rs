// 1419. Minimum Number of Frogs Croaking
// --------------------------------------

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut available = 0;
        let mut working = 0;
        let mut freq = vec![0; 5];

        for i in 0..croak_of_frogs.len() {
            match &croak_of_frogs[i..i + 1] {
                "c" => {
                    freq[0] += 1;
                    for j in 1..=4 {
                        if freq[j] > freq[0] {
                            return -1;
                        }
                    }
                    if available > 0 {
                        available -= 1;
                    }
                }
                "r" => {
                    freq[1] += 1;
                    for j in 0..1 {
                        if freq[j] < freq[1] {
                            return -1;
                        }
                    }
                    for j in 2..=4 {
                        if freq[j] > freq[1] {
                            return -1;
                        }
                    }
                }
                "o" => {
                    freq[2] += 1;
                    for j in 0..2 {
                        if freq[j] < freq[2] {
                            return -1;
                        }
                    }
                    for j in 3..=4 {
                        if freq[j] > freq[2] {
                            return -1;
                        }
                    }
                }
                "a" => {
                    freq[3] += 1;
                    for j in 0..3 {
                        if freq[j] < freq[3] {
                            return -1;
                        }
                    }
                    if freq[4] > freq[3] {
                        return -1;
                    }
                }
                "k" => {
                    for j in 0..4 {
                        if freq[j] < freq[4] {
                            return -1;
                        }
                    }
                    freq[4] += 1;
                    available += 1;
                }
                _ => {
                    return -1;
                }
            }
        }

        let s = freq[0];
        if freq.iter().all(|&f| f == s) == false {
            return -1;
        }

        available
    }
}
