package leetcode

/*
    1282. Group the People Given the Group Size They Belong To
    ----------------------------------------------------------
*/

func groupThePeople(groupSizes []int) [][]int {
    current := make(map[int][]int)
    res := [][]int{}

    for k := range groupSizes {
        size := groupSizes[k]
        if val, ok := current[size]; ok {
            current[size] = append(val, k)
        } else {
            current[size] = []int{k}
        }

        if len(current[size]) == size {
            res = append(res, current[size])
            current[size] = []int{}
        }
    }

    return res
}
