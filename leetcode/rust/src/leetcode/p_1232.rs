// 1232. Check If It Is a Straight Line
// ------------------------------------

#[derive(PartialEq, Debug)]

enum Orientation {
    Diagonal(f32),
    Ox(i32),
    Oy(i32),
    Invalid,
}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let orientation = Solution::determine_orientation(&coordinates, 0, 1);
        if orientation == Orientation::Invalid {
            return false;
        }

        for i in 2..coordinates.len() {
            let or = Solution::determine_orientation(&coordinates, i, i - 1);
            if or != orientation {
                return false;
            }
        }
        true
    }

    pub fn determine_orientation(coordinates: &Vec<Vec<i32>>, i: usize, j: usize) -> Orientation {
        let x0 = coordinates[i][0];
        let y0 = coordinates[i][1];
        let x1 = coordinates[j][0];
        let y1 = coordinates[j][1];

        if (x0 - x1).abs() > 0 && (y0 - y1).abs() > 0 {
            let slope = (x0 - x1) as f32 / (y0 - y1) as f32;
            return Orientation::Diagonal(slope);
        }

        if (x0 - x1).abs() > 0 {
            return Orientation::Ox(y0);
        }

        if (y0 - y1).abs() > 0 {
            return Orientation::Oy(x0);
        }
        Orientation::Invalid
    }
}
