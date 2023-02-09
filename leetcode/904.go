package leetcode

func totalFruits(fruits []int) int {

    total := 0
    length := len(fruits)

    if length <= 2 {
        return length
    }

    for i := 0; i < length - 1; i++ {
        counter := 0
        sub := fruits[i:]        
        first := sub[0]
        second := sub[1]
        for j := 0; j < len(sub); j++ {
            if sub[j] != first && sub[j] != second {
                if first == second {
                    second = sub[j]
                } else {
                    break
                }
            }
            counter++
        }
        if total < counter {
            total = counter
        }
        if total >= len(sub) {
            break
        }
    }
    
    return total
}