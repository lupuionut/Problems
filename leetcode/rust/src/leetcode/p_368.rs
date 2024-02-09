// 368. Largest Divisible Subset
// -----------------------------

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        fn get_max(
            i: usize,
            nums: &Vec<i32>,
            best_moves: &mut Vec<Option<usize>>,
            cache: &mut Vec<i32>,
        ) -> i32 {
            let mut best = 1;

            if cache[i] != -1 {
                return cache[i];
            }
            for j in i + 1..nums.len() {
                if nums[j] % nums[i] == 0 {
                    let next_best = 1 + get_max(j, nums, best_moves, cache);
                    if next_best > best {
                        best = next_best;
                        best_moves[i] = Some(j);
                    }
                }
            }

            cache[i] = best;
            cache[i]
        }

        fn get_best_move(
            i: usize,
            best_moves: &Vec<Option<usize>>,
            nums: &Vec<i32>,
            ans: &mut Vec<i32>,
        ) {
            ans.push(nums[i]);
            if let Some(nxt) = best_moves[i] {
                get_best_move(nxt, best_moves, nums, ans);
            }
        }

        let mut nums = nums;
        nums.sort();
        let mut best_moves = vec![None; nums.len()];
        let mut best_start = 0;
        let mut best = 0;
        let mut ans = vec![];
        let mut cache = vec![-1; nums.len()];

        for i in 0..nums.len() {
            let nxt = get_max(i, &nums, &mut best_moves, &mut cache);
            if nxt > best {
                best = nxt;
                best_start = i;
            }
        }
        get_best_move(best_start, &best_moves, &nums, &mut ans);
        ans
    }
}
