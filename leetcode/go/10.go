package leetcode

func IsMatch(s string, p string) bool {
    if len(p) == 0 {
        if len(s) == 0 {
            return true
        }
        return false
    }

    fm := false
    if len(s) > 0 && (p[0] == s[0] || p[0] == '.') {
        fm = true
    }

    if len(p) >= 2 && p[1] == '*' {
        return IsMatch(s, p[2:]) || (fm && IsMatch(s[1:],p))
    } else{
        return fm && IsMatch(s[1:], p[1:])
    }
}
