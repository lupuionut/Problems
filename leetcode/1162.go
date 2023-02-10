package leetcode

//import "fmt"

func MaxDistance(grid [][]int) int {
    max := -1
    zeros := []*Point{}
    ones := []*Point{}

    for y := range(grid) {
        for x := range(grid[y]) {
            if grid[y][x] == 0 {
               zeros = append(zeros, &Point{X: x, Y: y})
            } else {
                ones = append(ones, &Point{X:x, Y:y})
            }
        }
    }

    if len(zeros) == 0 || len(ones) == 0 {
        return max
    }

    mins := []int{}
        
    for i := 0; i < len(zeros); i++ {
        min := 0
        for j := 0; j < len(ones); j++ {
            d1 := zeros[i].X - ones[j].X
            d2 := zeros[i].Y - ones[j].Y
            if d1 < 0 {
                d1 = d1 * (-1)
            }
            if d2 < 0 {
                d2 = d2 * (-1)
            }
            distance := d1 + d2
            if min == 0 || distance < min {
                min = distance
            }
        }
        mins = append(mins, min)
    }

    for _, min := range(mins) {
        if min > max {
            max = min
        }
    }

    return max
}

type Point struct {
    X int
    Y int
}
