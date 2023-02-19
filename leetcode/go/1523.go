package leetcode

func CountOdds(low int, high int) int {
    count := 0
    delta := high - low 
    
    if low & 1 == 0 && high & 1 == 0 {
        count = delta/2
    } else {
        count = delta/2 + 1
    }

    return count
}