// 401. Binary Watch
// -----------------

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut ans = vec![];

        fn on(mut x: i32) -> i32 {
            let mut count = 0;
            while x > 0 {
                count += x % 2;
                x /= 2;
            }
            count
        }

        fn stringify(h: i32, m: i32) -> String {
            let prefixes = ["00", "01", "02", "03", "04", "05", "06", "07", "08", "09"];
            let mut hour = h.to_string();
            let mut minutes = String::new();
            if m < 10 {
                minutes = prefixes[m as usize].to_string();
            } else {
                minutes = m.to_string();
            }
            format!("{}:{}", hour, minutes)
        }

        for hour in 0..12 {
            for minutes in 0..60 {
                if on(hour) + on(minutes) == turned_on {
                    ans.push(stringify(hour, minutes));
                }
            }
        }

        ans
    }
}
