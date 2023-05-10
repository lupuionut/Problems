package leetcode

/*
    1103. Distribute Candies to People
    ----------------------------------
*/

func distributeCandies(candies int, num_people int) []int {
    cursor, total := 0, 0
    ans := make([]int, num_people)

    loop:
    for {
        for i := 0; i < num_people; i++ {
            cursor += 1
            total += cursor

            if total >= candies {
                delta := total - candies
                cursor -= delta
                ans[i] += cursor
                break loop
            }
            ans[i] += cursor
        }
    }
    return ans
}
