// 2103. Rings and Rods
// --------------------
// R G B     R G B
// 0 0 0     1 1 1 = 7

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut frame = rings.as_bytes().windows(2);
        let mut rods = vec![0; 10];

        while let Some(pair) = frame.next() {
            let k = (pair[1] - 48) as usize;
            match pair[0] {
                b'R' => rods[k] = rods[k] | (1 << 2),
                b'G' => rods[k] = rods[k] | (1 << 1),
                b'B' => rods[k] = rods[k] | (1 << 0),
                _ => {}
            }
        }

        let mut ans = rods
            .iter()
            .fold(0, |acc, v| if *v == 7 { acc + 1 } else { acc });
        ans
    }
}
