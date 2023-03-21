package leetcode

func IsMatch(s string, p string) bool {
    if len(p) == 0 {
        if len(s) == 0 {
            return true
        }
        return false
    }

    match := false

    // first check if we match the current char
    if len(s) > 0 && (p[0] == s[0] || p[0] == '.') {
        match = true
    }

    // if next char of pattern is * we have 2 options
    // either we dont use the * and we compare s with p[2:]
    // or we use the * and we compare s[1:] with p
    if len(p) >= 2 && p[1] == '*' {
        return IsMatch(s, p[2:]) || (match && IsMatch(s[1:],p))
    } else{
        return match && IsMatch(s[1:], p[1:])
    }
}
