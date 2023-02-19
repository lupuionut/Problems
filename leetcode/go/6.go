package leetcode

func Convert(s string, numRows int) string {
    result := ""
    rows := make([]string, numRows)
    length := numRows - 1
    cycle := 2 * length
    
    for i := 0; i < len(s); i++ {
        row := 0
        if numRows > 1 {
            row = i % length
            if row != 0 {
                i1 := i % cycle
                if row < i1 {
                    row = cycle - i1
                }
            } else {
                if i % cycle == 0 {
                    row = 0
                } else {
                    row = length
                }
            }
        }
        rows[row] = rows[row] + string(s[i])
    }
    for _, w := range(rows) {
        result += w
    }
    return result
}