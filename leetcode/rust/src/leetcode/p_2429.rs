// 2429. Minimize XOR
// ------------------

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        // get binary representation
        let mut num1 = format!("{:b}", num1);
        let mut num2 = format!("{:b}", num2);

        // find how many ones we must use
        let mut ones = 0;
        num2.chars().for_each(|c| {
            if c == '1' {
                ones += 1;
            }
        });

        // we must use a one where there is a one in num1
        let mut x = num1
            .chars()
            .map(|c| {
                if ones > 0 {
                    if c == '1' {
                        ones -= 1;
                        '1'
                    } else {
                        '0'
                    }
                } else {
                    '0'
                }
            })
            .collect::<Vec<_>>();

        // if we have remaining ones, try to insert in the position
        // that will give us the min value
        while ones > 0 {
            let mut inserted = false;
            for i in (0..x.len()).rev() {
                if x[i] == '0' {
                    x[i] = '1';
                    inserted = true;
                    break;
                }
            }
            if inserted == false {
                x.push('1');
            }
            ones -= 1;
        }

        // get the value from the vec bin representation
        let mut ans = 0;
        let n = x.len() - 1;
        for i in 0..x.len() {
            if x[i] == '1' {
                ans += (1 << (n - i) as i32);
            }
        }

        ans
    }
}
