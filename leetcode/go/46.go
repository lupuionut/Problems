package leetcode

/*
    46. Permutations
    ----------------
*/

func permute(nums []int) [][]int {

    if len(nums) == 1 {
        return [][]int{{nums[0]}}
    }
    if len(nums) == 2 {
        return [][]int{{nums[0], nums[1]}, {nums[1], nums[0]}}
    }

    t := [][]int{}
    for i := 0; i < len(nums); i++ {
        head := nums[i]
        tail := append([]int{}, nums[0:i]...)
        tail = append(tail, nums[i+1:]...)
        r := permute(tail)
        for k := range r {
            t = append(t, append([]int{head}, r[k]...))
        }
    }

    return t
}
