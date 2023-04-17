package leetcode

/*
    1431. Kids With the Greatest Number of Candies
    ----------------------------------------------
*/
func kidsWithCandies(candies []int, extraCandies int) []bool {
    max := candies[0]
    for k := range candies {
        if candies[k] > max {
            max = candies[k]
        }
    }

    res := []bool{}
    for k := range candies {
        if candies[k] + extraCandies >= max {
            res = append(res, true)
        } else {
            res = append(res, false)
        }
    }

    return res
}
