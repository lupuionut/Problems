// 2140. Solving Questions With Brainpower
// ---------------------------------------

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        fn best(i: usize, qs: &Vec<Vec<i32>>, cache: &mut Vec<i64>) -> i64 {
            if i >= qs.len() {
                return 0;
            }
            if cache[i] != -1 {
                return cache[i];
            }

            let mut max_points = 0;

            // take
            let points = qs[i][0] as i64;
            let skip = qs[i][1] as usize + 1;
            max_points = points + best(i + skip, qs, cache);

            // skip
            max_points = max_points.max(best(i + 1, qs, cache));

            cache[i] = max_points;
            max_points
        }

        let mut cache = vec![-1i64; questions.len()];
        best(0, &questions, &mut cache)
    }
}
