// 636. Exclusive Time of Functions
// --------------------------------

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        let mut stack: Vec<(i32, i32)> = vec![];
        let mut prev_start_time = 0;

        // return [id, type, val]
        fn parse_log(log: &str) -> Vec<i32> {
            log.split(':')
                .map(|c| {
                    if c == "start" {
                        0
                    } else if c == "end" {
                        1
                    } else {
                        c.parse::<i32>().unwrap()
                    }
                })
                .collect()
        }

        for log in &logs {
            let line = parse_log(log);
            let log_fid = line[0];
            let log_type = line[1];
            let log_val = line[2];

            // if we have a start
            if log_type == 0 {
                let n = stack.len();
                // update previous function from stack with the existing interval
                if n > 0 {
                    let (id, val) = stack[n - 1];
                    ans[id as usize] += log_val - prev_start_time;
                }
                stack.push((log_fid, log_val));
                prev_start_time = log_val;
            } else {
                // update function time on end with the current interval time
                // and update prev_start_time with current end time
                let (id, val) = stack.pop().unwrap();
                let intval = log_val - prev_start_time + 1;
                ans[id as usize] += intval;
                prev_start_time = log_val + 1;
            };
        }

        ans
    }
}
