package leetcode

/*
    2611. Mice and Cheese
    ---------------------
    Make a map with index the difference between reward1 and reward2 and
    values the indices for these differences.
    Total = take k values from reward1 starting from highest difference +
    remaining total from rewards2 excluding the indices used in reward1.
*/

func miceAndCheese(reward1 []int, reward2 []int, k int) int {

    max, min := 0, 1001
    sorted := make(map[int][]int)
    reward2_total := 0

    for i := range reward1 {
        delta := reward1[i] - reward2[i]
        if _, ok := sorted[delta]; !ok {
            sorted[delta] = []int{}
        }
        if delta > max {
            max = delta
        }
        if delta < min {
            min = delta
        }
        sorted[delta] = append(sorted[delta], i)
        reward2_total += reward2[i]
    }

    total := 0

    loop:
    for i := max; i >= min; i-- {
        if indexes, ok := sorted[i]; ok {
            for j := range indexes {
                idx := indexes[j]
                if k == 0 {
                    total += reward2_total
                    break loop
                }
                total += reward1[idx]
                reward2_total -= reward2[idx]
                k -= 1
            }
        }
    }

    return total
}
