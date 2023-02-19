mod leetcode;

fn main() {
    let input = vec![2, 5, 1, 3, 4, 7];
    let n = 3;
    let a = leetcode::p_1470::shuffle(input, n);
    println!("{:?}", a)
}
