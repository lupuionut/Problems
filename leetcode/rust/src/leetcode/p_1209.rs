// 1209. Remove All Adjacent Duplicates in String II
// -------------------------------------------------

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<u8> = vec![];
        let frame = (k - 1) as usize;
        let mut s = s.as_bytes();

        s.iter().for_each(|c| {
            if stack.len() >= frame {
                let iter = stack.windows(frame);
                if let Some(last) = iter.last() {
                    let mut same_all = true;
                    for i in 0..last.len() {
                        if last[i] != *c {
                            same_all = false;
                            break;
                        }
                    }
                    if same_all {
                        let end = stack.len() - last.len();
                        stack = stack.drain(0..end).collect();
                    } else {
                        stack.push(*c);
                    }
                }
            } else {
                stack.push(*c);
            }
        });

        String::from_utf8(stack).unwrap()
    }
}
