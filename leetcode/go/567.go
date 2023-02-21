package leetcode

// import "fmt"

func CheckInclusion(s1 string, s2 string) bool {
    if len(s2) < len(s1) {
        return false
    }

    result := false
    keys := make(map[rune]int)
    foundkeys := make(map[rune]int)
    for _, v := range(s1) {
        if count, ok := keys[v]; ok {
            keys[v] = count + 1
        } else {
            keys[v] = 1
        }
    }

    loop:
    for i := 0; i <= len(s2) - len(s1); i++ {
        foundkeys = CheckInclusionCopyMap(keys, foundkeys)
        substring := s2[i:i+len(s1)]
        for _, c := range(substring) {
            if _, ok := keys[c]; ok {
                foundkeys[c] -= 1
            } else {
                continue loop
            }
        }
        if CheckInclusionFound(foundkeys) {
            return true
        }
    }

    return result
}

func CheckInclusionFound(a map[rune]int) bool {
    for _, k := range(a) {
        if k != 0 {
            return false
        }
    }
    return true
}

func CheckInclusionCopyMap(b, a map[rune]int) map[rune]int {
    for k, v := range(b) {
        a[k] = v
    }
    return a
}
