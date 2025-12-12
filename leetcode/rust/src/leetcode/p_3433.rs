// 3433. Count Mentions Per User
// -----------------------------
impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut ans = vec![0; number_of_users as usize];
        let mut online = vec![0; number_of_users as usize];
        let mut order: Vec<(i32, i32, usize)> = events
            .iter()
            .enumerate()
            .map(|(k, event)| {
                let t = event[1].parse::<i32>().unwrap();
                let o = if event[0] == "OFFLINE".to_string() {
                    0
                } else {
                    1
                };
                return (t, o, k);
            })
            .collect();
        order.sort();
        for i in 0..order.len() {
            let time = order[i].0;
            let key = order[i].2;
            if events[key][0] == "OFFLINE".to_string() {
                let id = events[key][2].parse::<i32>().unwrap() as usize;
                online[id] = time + 60;
            } else {
                if events[key][2] == "HERE".to_string() {
                    for j in 0..ans.len() {
                        if online[j] <= time {
                            ans[j] += 1;
                        }
                    }
                } else if events[key][2] == "ALL".to_string() {
                    for j in 0..ans.len() {
                        ans[j] += 1;
                    }
                } else {
                    let p: String = events[key][2].split(" ").collect();
                    let p: Vec<_> = p.split("id").collect();
                    for j in 1..p.len() {
                        let k = p[j].parse::<usize>().unwrap();
                        ans[k] += 1;
                    }
                }
            }
        }
        ans
    }
}
