package leetcode

/*
    1190. Reverse Substrings Between Each Pair of Parentheses
    ---------------------------------------------------------
    We have 3 situations: we meet a "(" a ")" or a letter.
    In case we meet a "(", this mean we should put it on stack.
    In case we meet a ")", we must reverse the last item on stack
    and if we have another item on stack, concat them.
    In case we meet a letter, just concat to the last item on stack.
*/

func reverseParentheses(s string) string {
    stack := []string{}

    takeInner := func(i int) (string, int) {
        word := []byte{}
        stop := i
        for j := i; j < len(s); j++ {
            if s[j] == '(' || s[j] == ')' {
                return string(word), j - 1
            } else {
                word = append(word, s[j])
            }
            stop = j
        }
        return string(word), stop - 1
    }

    reverse := func(w string) string {
        b := []byte(w)
        r := []byte{}
        for i := len(b) - 1; i >= 0; i-- {
            r = append(r, b[i])
        }
        return string(r)
    }

    for i := 0; i < len(s); i++ {
        if s[i] == '(' {
            w, j := takeInner(i+1)
            i = j
            stack = append(stack, w)
            continue
        }
        if s[i] == ')' {
            l := len(stack)
            last := reverse(stack[l-1])

            if l > 1 {
                last = stack[l-2] + last
                stack = append(stack[0:l-2], last)
            } else {
                stack = append(stack[0:l-1], last)
            }
            continue
        }
        if len(stack) > 0 {
            stack[len(stack)-1] += string(s[i])
        } else {
            stack = append(stack, string(s[i]))
        }
    }

    return stack[0]
}
