package leetcode

/*
    2300. Successful Pairs of Spells and Potions
    --------------------------------------------
*/

import "sort"

func successfulPairs(spells []int, potions []int, success int64) []int {
    pairs := make([]int, len(spells))
    orig := append([]int{}, spells...)
    hm := make(map[int]int)

    sort.Ints(spells)
    sort.Ints(potions)
    for s := range spells {
        if _, ok := hm[spells[s]]; !ok {
            if spells[s] >= int(success) {
                hm[spells[s]] = len(potions)
                continue
            }

            count := 0
            for p := range potions {
                if int64(spells[s] * potions[p]) >= success {
                    count = len(potions) - p
                    break
                }
            }
            hm[spells[s]] = count
        }
    }

    for k := range orig {
        key := orig[k]
        if val, ok := hm[key]; ok {
            pairs[k] = val
        }
    }

    return pairs
}
