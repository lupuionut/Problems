package leetcode

func MinimumTime(time []int, totalTrips int) int {

    max := func(time []int) int {
        m := time[0]
        for i := 0; i < len(time); i++ {
            if time[i] > m {
                m = time[i]
            }
        }
        return m
    }

    isLargeEnough := func(n int) bool {
        count := 0
        for i := 0; i < len(time); i++ {
            count += n/time[i]
            if count >= totalTrips {
                return true
            }
        }
        return false
    }

    left := 1
    right := max(time) * totalTrips

    for true {
        if left == right {
            break
        }
        mid := (left + right)/2
        if isLargeEnough(mid) {
            right = mid
        } else {
            left = mid + 1
        }
    }

    return left
}
