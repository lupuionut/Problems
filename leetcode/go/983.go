package leetcode

//import "fmt"

func MincostTickets(days []int, costs []int) int {

    d := make([]int, 365)
    m := make([]int, 365)

    for i := 0; i < 365; i++ {
        m[i] = -1
    }

    for k := range days {
        d[days[k] - 1] = days[k]
    }

    return MincostTicketsRec(d, costs, m, 0)
}

func MincostTicketsRec(days []int, costs []int, m []int, day int) int {
    if day >= 365 {
        return 0
    }

    if m[day] != -1 {
        return m[day]
    }

    min3 := func(a,b,c int) int {
        min := a
        if b < min {
            min = b
        }
        if c < min {
            min = c
        }
        return min
    }

    if days[day] == 0 {
        m[day] = MincostTicketsRec(days, costs, m, day + 1)
    } else {
        m[day] = min3(
            costs[0] + MincostTicketsRec(days, costs, m, day + 1),
            costs[1] + MincostTicketsRec(days, costs, m, day + 7),
            costs[2] + MincostTicketsRec(days, costs, m, day + 30),
        )
    }
    return m[day]
}
