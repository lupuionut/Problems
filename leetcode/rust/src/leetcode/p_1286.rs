//1286. Iterator for Combination

struct CombinationIterator {
    index: usize,
    combinations: Vec<String>,
    max_combinations: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combinationLength: i32) -> Self {
        let mut chars: Vec<char> = Vec::new();
        let mut ans: Vec<String> = Vec::new();
        CombinationIterator::combinations(
            &characters,
            &mut chars,
            0,
            combinationLength as usize,
            &mut ans,
        );
        let max = ans.len();

        CombinationIterator {
            index: 0,
            combinations: ans,
            max_combinations: max,
        }
    }

    fn next(&mut self) -> String {
        let ans = &self.combinations[self.index as usize];
        self.index += 1;
        ans.to_string()
    }

    fn has_next(&self) -> bool {
        if self.index < self.max_combinations {
            return true;
        }
        false
    }

    fn combinations(
        input: &str,
        comb: &mut Vec<char>,
        start: usize,
        k: usize,
        ans: &mut Vec<String>,
    ) -> () {
        if comb.len() == k {
            ans.push(comb.iter().collect::<String>());
            return;
        }
        let chars = input.get(start..);
        if chars.is_some() {
            let mut i = start;
            for c in chars.unwrap().chars() {
                i += 1;
                comb.push(c);
                CombinationIterator::combinations(input, comb, i, k, ans);
                comb.pop();
            }
        }
    }
}
