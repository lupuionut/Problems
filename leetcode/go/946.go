package leetcode

/*
    946. Validate Stack Sequences
    -----------------------------
    Push to the stack and check on each push if it's the value that we need
    to pop (popped[0]). If it is, try to pop as much as possible, if not continue
    to add to the stack.
    If we popped everything, length of popped should be 0.
*/

func validateStackSequences(pushed []int, popped []int) bool {

    stack := []int{}

    for k := range pushed {
        stack = append(stack, pushed[k])
        last := len(stack) - 1
        for last >= 0 && len(stack) > 0 && len(popped) > 0 && stack[last] == popped[0] {
            stack = stack[0:last]
            popped = popped[1:]
            last = last - 1
        }
    }
    if len(popped) == 0 {
        return true
    }
    return false
}
