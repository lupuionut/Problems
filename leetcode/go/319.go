package leetcode

/*
    319. Bulb Switcher
    ------------------
*/

import "math"

func bulbSwitch(n int) int {
    return int(math.Sqrt(float64(n)))
}
