package leetcode

func MinEatingSpeed(piles []int, h int) int {

    if len(piles) == 1 {
        if piles[0]%h != 0 {
            return piles[0]/h + 1
        }
        return piles[0]/h
    }

    overHours := func(piles []int, quantity int) bool {
        hours := 0
        for k := range piles {
            hours += piles[k]/quantity
            if piles[k]%quantity != 0 {
                hours += 1
            }
            if hours > h {
                return true
            }
        }
        return false
    }

    max := func(piles []int) int {
        m := 0
        for k := range piles {
            if piles[k] > m {
                m = piles[k]
            }
        }
        return m
    }

    min := func(piles []int) int {
        m := 0
        for k := range piles {
            if piles[k] < m {
                m = piles[k]
            }
        }
        return m
    }

    left := min(piles)
    right := max(piles)

    for true {
        if left == right {
            break
        }

        mid := (left+right)/2
        if mid == 0 {
            return 1
        }
        if overHours(piles, mid) {
            left = mid + 1
        } else {
            right = mid
        }
    }

    return left
}
