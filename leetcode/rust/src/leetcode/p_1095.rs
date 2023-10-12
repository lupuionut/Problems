// 1095. Find in Mountain Array
// ----------------------------

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        fn find_peak(m: &MountainArray) -> i32 {
            let mut l = 0;
            let length = m.length() - 1;
            let mut r = length;

            while l <= r {
                let middle = (l + r) / 2;
                let mv = m.get(middle);

                if middle <= 0 {
                    return 1;
                }

                if middle >= length {
                    return length - 1;
                }

                let lv = m.get(middle - 1);
                let rv = m.get(middle + 1);

                if mv > lv && mv > rv {
                    return middle;
                } else {
                    if mv < lv {
                        r = middle - 1;
                    } else {
                        l = middle + 1;
                    }
                }
            }

            -1
        }

        fn find_range(l: i32, r: i32, m: &MountainArray, target: i32, direction: i32) -> i32 {
            let mut left = l;
            let mut right = r;

            while left <= right {
                let middle = (left + right) / 2;
                if middle < l || middle > r {
                    return -1;
                }

                let mv = m.get(middle);

                if mv == target {
                    return middle;
                } else {
                    // left
                    if direction == 1 {
                        if mv > target {
                            right = middle - 1;
                        } else {
                            left = middle + 1;
                        }
                    // right
                    } else {
                        if mv > target {
                            left = middle + 1;
                        } else {
                            right = middle - 1;
                        }
                    }
                }
            }

            -1
        }

        let peak = find_peak(mountainArr);
        let left = find_range(0, peak, mountainArr, target, 1);
        if left != -1 {
            return left;
        } else {
            return find_range(peak, mountainArr.length() - 1, mountainArr, target, -1);
        }
    }
}
