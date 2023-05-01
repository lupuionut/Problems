package leetcode

/*
    1491. Average Salary Excluding the Minimum and Maximum Salary
    -------------------------------------------------------------
*/

func average(salary []int) float64 {
    min, max := 1000000, 1000
    n := len(salary) - 2
    sum := 0
    for k := range salary {
        if salary[k] > max {
            max = salary[k]
        }
        if salary[k] < min {
            min = salary[k]
        }
        sum += salary[k]
    }
    return float64(sum - min - max)/float64(n)
}
