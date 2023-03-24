package leetcode

func maxArea(height []int) int {
    volume := func(start, end int) int {
        delta := end - start
        unit := height[start]
        if height[end] < unit {
            unit = height[end]
        }
        return unit * delta
    }

    max := 0
    i := 0
    j := len(height) - 1

    for {
        if i == j {
            break
        }
        vol := volume(i, j)
        if vol > max {
            max = vol
        }
        if height[i] < height[j] {
            i += 1
        } else {
            j -= 1
        }
    }

    return max
}
