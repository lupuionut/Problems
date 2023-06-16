// 1569. Number of Ways to Reorder Array to Get Same BST
// -----------------------------------------------------
// Read the editorial, use Pascal's triangle to calculate nCk

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        fn pascal_triangle(n: usize) -> Vec<Vec<u64>> {
            let mut triangle: Vec<Vec<u64>> = Vec::new();
            for i in 0..=n {
                let mut t = vec![1; i + 1];
                triangle.push(t);
                for j in 1..i {
                    triangle[i][j] = (triangle[i - 1][j - 1] + triangle[i - 1][j]) % 1_000_000_007;
                }
            }
            triangle
        }

        fn dp(tree: Vec<i32>, triangle: &Vec<Vec<u64>>) -> u64 {
            if tree.len() <= 2 {
                return 1;
            }
            let mut left: Vec<i32> = vec![];
            let mut right: Vec<i32> = vec![];

            for i in 0..tree.len() {
                if tree[i] > tree[0] {
                    right.push(tree[i]);
                }
                if tree[i] < tree[0] {
                    left.push(tree[i]);
                }
            }

            let n = tree.len() - 1;
            let k = left.len();

            return ((dp(left, triangle) * dp(right, triangle)) % 1_000_000_007
                * (triangle[n][k] % 1_000_000_007))
                % 1_000_000_007;
        }

        let triangle = pascal_triangle(nums.len());
        ((dp(nums, &triangle) - 1) % 1_000_000_007) as i32
    }
}
