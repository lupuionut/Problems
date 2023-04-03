package leetcode

func longestValidParentheses(s string) int {
    max := 0
    stack := []int{-1}

    for i := 0; i < len(s); i++ {
        if s[i] == '(' {
            stack = append(stack, i)
        } else {
            l := len(stack) - 1
            stack = stack[0:l]
            if len(stack) == 0 {
                stack = append(stack, i)
            } else {
                m := i - stack[l - 1]
                if m > max {
                    max = m
                }
            }
        }
    }

    return max
}
