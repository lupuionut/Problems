package leetcode

/*
    20. Valid Parentheses
    ---------------------
    First, check if the length is odd, that means we cannot have valid parantheses.

    Keep track of open brackets by adding corresponding closing bracket into a slice.
    Whenever a closing brackets occurs, check if the slice (st) last elem is
    the same as the closing bracket that appears.
*/

func isValid(s string) bool {
    if len(s) & 1 == 1 {
        return false
    }
    brackets := make(map[byte]byte)
    brackets['('] = ')'
    brackets['['] = ']'
    brackets['{'] = '}'
    st := []byte{}

    for k := range s {
        c := s[k]
        if val, ok := brackets[c]; ok {
            st = append(st, val)
        } else {
            l := len(st) - 1
            if l == -1 || c != st[l] {
                return false
            }
            st = st[0:l]
        }
    }

    if len(st) != 0 {
        return false
    }
    return true
}
