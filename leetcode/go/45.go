package leetcode

func Jump(nums []int) int {
    steps := 0
    length := len(nums) - 1

    if length < 1 {
        return 0
    }

    for i := 0; i <= length; {

        if i == length {
            break
        }
        current := nums[i]
        distance := current + i
        steps += 1

        if distance >= length {
            i = length
            break
        }

        max_i := current
        max_v := nums[i+current]

        for k := 1; k <= current; k++ {
            left := k + nums[i+k]
            right := max_i + max_v
            if left > right {
                max_v = nums[i+k]
                max_i = k
            }
        }

        i += max_i
    }
    return steps
}
