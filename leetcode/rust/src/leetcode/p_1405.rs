// 1405. Longest Happy String
// --------------------------

use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap = BinaryHeap::new();
        let mut st = vec![];
        let mut sequence = vec!['0', '1'];
        heap.push((a, 'a'));
        heap.push((b, 'b'));
        heap.push((c, 'c'));

        loop {
            if let Some(max) = heap.pop() {
                // if we completely used all 3 chars, stop
                if max.0 == 0 {
                    break;
                }

                // we have already a sequence of same 2 chars
                if sequence[0] == sequence[1] && sequence[0] == max.1 {
                    // take the next char
                    if let Some(next) = heap.pop() {
                        // if next char is already completely used, stop
                        if next.0 == 0 {
                            break;
                        }
                        sequence[0] = sequence[1];
                        sequence[1] = next.1;
                        st.push(next.1);
                        heap.push((next.0 - 1, next.1));
                    } else {
                        break;
                    }
                    heap.push(max);
                } else {
                    sequence[0] = sequence[1];
                    sequence[1] = max.1;
                    st.push(max.1);
                    heap.push((max.0 - 1, max.1));
                }
            }
        }

        st.iter().collect::<String>()
    }
}
