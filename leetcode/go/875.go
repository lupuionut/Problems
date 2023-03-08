package leetcode
/*
    Do a binary search and see which is the values of banans
    that needs to be eaten hourly so that we can fit in the hours constraint
*/
func MinEatingSpeed(piles []int, h int) int {

    if len(piles) == 1 {
        if piles[0]%h != 0 {
            return piles[0]/h + 1
        }
        return piles[0]/h
    }

    // if the quantity is too small, and we need more hours return true
    overHours := func(quantity int) bool {
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

    boundaries := func() (int,int) {
        mn := 0
        mx := 0
        for k := range piles {
            if piles[k] > mx {
                mx = piles[k]
            }
            if piles[k] < mn {
                mn = piles[k]
            }
        }
        return mn, mx
    }

    left, right := boundaries()

    for true {
        if left == right {
            break
        }

        mid := (left+right)/2
        if mid == 0 {
            return 1
        }
        // if the quantity of bananas is too little, move left to mid
        if overHours(mid) {
            left = mid + 1
        } else {
            right = mid
        }
    }

    return left
}
