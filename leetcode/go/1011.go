package leetcode

/*
    1011. Capacity To Ship Packages Within D Days
    ---------------------------------------------
    The minimum capacity is the maximum weight.
    The maximum capacity is the sum of all weights.
    Run binary search to check the optimal weight between
    min and max capacity.
*/

func shipWithinDays(weights []int, days int) int {
    l := weights[0]
    r := 0

    for k := range weights {
        r += weights[k]
        if weights[k] > l {
            l = weights[k]
        }
    }

    canShip := func(w int) bool {
        d := 1
        t := 0
        for k := range weights {
            t += weights[k]
            if t > w {
                t = weights[k]
                d += 1
            }
        }
        if d <= days {
            return true
        }
        return false
    }

    for l < r {
        mid := (l + r) / 2
        if canShip(mid) {
            r = mid
        } else {
            l = mid + 1
        }
    }

    return r
}
