// 1601. Maximum Number of Achievable Transfer Requests
// ----------------------------------------------------

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let mut net_change = vec![0; n as usize];

        fn backtrack(i: usize, requests: &Vec<Vec<i32>>, net_change: &mut Vec<i32>) -> i32 {
            if i == requests.len() {
                if *net_change.iter().max().unwrap() == 0 {
                    return 0;
                } else {
                    return std::i32::MIN;
                }
            }

            let mut res = std::i32::MIN;
            let from = requests[i][0] as usize;
            let to = requests[i][1] as usize;

            if from == to {
                return 1 + backtrack(i + 1, requests, net_change);
            } else {
                net_change[from] -= 1;
                net_change[to] += 1;

                res = res.max(1 + backtrack(i + 1, requests, net_change));

                net_change[from] += 1;
                net_change[to] -= 1;
                res = res.max(backtrack(i + 1, requests, net_change));
                return res;
            }
        }

        backtrack(0, &requests, &mut net_change)
    }
}
