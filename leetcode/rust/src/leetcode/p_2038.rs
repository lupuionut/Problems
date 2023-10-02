// 2038. Remove Colored Pieces if Both Neighbors are the Same Color
// ----------------------------------------------------------------

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut streak = 0;

        let alice_moves = colors.chars().fold(0, |acc, c| {
            if c == 'A' {
                streak += 1;
            } else {
                streak = 0;
            }

            if streak > 2 {
                acc + 1
            } else {
                acc
            }
        });

        streak = 0;

        let bob_moves = colors.chars().fold(0, |acc, c| {
            if c == 'B' {
                streak += 1;
            } else {
                streak = 0;
            }

            if streak > 2 {
                acc + 1
            } else {
                acc
            }
        });

        if alice_moves > bob_moves {
            true
        } else {
            false
        }
    }
}
