package leetcode

/*
    22. Generate Parentheses
    ------------------------
    We must keep track of each open/closed paranthesis.
    For each step, we have options:
    - if open are half of max, meaning we used all, we can only use closing paranthesis
    - if open == closed, we can only use open paranthesis since closed is invalid
    - if open > closed, we can either use open / closed paranthesis
*/

func generateParenthesis(n int) []string {
    return makeParanthesis([]string{"("}, 1, 0, 2 * n)
}

func makeParanthesis(previously []string, open, closed, max int) []string {
    if open + closed == max {
        return previously
    }

    if open == max/2 {
        temp := []string{}
        for k := range previously {
            temp = append(temp, previously[k] + ")")
        }
        return makeParanthesis(temp, open, closed + 1, max)
    }

    if open == closed {
        temp := []string{}
        for k := range previously {
            temp = append(temp, previously[k] + "(")
        }
        return makeParanthesis(temp, open + 1, closed, max)
    }

    if open > closed {
        v1 := []string{}
        v2 := []string{}
        for k := range previously {
            v1 = append(v1, previously[k] + "(")
            v2 = append(v2, previously[k] + ")")
        }
        t1 := makeParanthesis(v1, open + 1, closed, max)
        t2 := makeParanthesis(v2, open, closed + 1, max)
        temp := append(t1, t2...)
        return temp
    }

    return nil
}
