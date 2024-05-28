// 1024. Video Stitching
// ---------------------

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut ans = 0;
        let mut curr_max = 0;
        let mut prev_max = 0;
        let mut maxes = vec![0; 101];

        clips.iter().for_each(|clip| {
            maxes[clip[0] as usize] = maxes[clip[0] as usize].max(clip[1]);
        });

        loop {
            if curr_max >= time {
                break;
            }
            let start = prev_max;
            let end = curr_max;

            prev_max = curr_max;
            for i in start..=end {
                curr_max = curr_max.max(maxes[i as usize]);
            }

            if prev_max == curr_max {
                break;
            }

            ans += 1;
        }

        if curr_max < time {
            return -1;
        }

        ans
    }
}
