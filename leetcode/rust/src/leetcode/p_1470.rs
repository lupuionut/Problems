pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let (first, second) = nums.split_at(n as usize);
    let mut result = vec![];
    for i in 0..n as usize {
        result.insert(i * 2, first[i]);
        result.insert(i * 2 + 1, second[i]);
    }
    result
}
