package leetcode

//import "fmt"

func IsAlienSorted(words []string, order string) bool {
    weights := make(map[rune]int)
    for k, v := range(order) {
        weights[v] = k
    }

    if len(words) == 1 {
        return true
    }
    
    for i := 0; i < len(words) - 1; i++ {
        if compareWords(words[i], words[i+1], weights) == false {
            return false
        }
    }
    
    return true
}

// return true if a < b, false if a > b
func compareWords(a, b string, weights map[rune]int) bool {
    min := len(a)
    if len(b) < min {
        min = len(b)
    }
    for i := 0; i < min; i++ {
        fst := weights[rune(a[i])]
        sec := weights[rune(b[i])]
        if fst == sec {
            continue
        } else {
            if fst < sec {
                return true
            } else {
                return false
            }
        }
    }
    if len(a) > min {
        return false
    }
    return true
}