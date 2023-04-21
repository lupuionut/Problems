package leetcode

/*
    79. Word Search
    ---------------
*/

func exist(board [][]byte, word string) bool {

    m := len(board[0])
    n := len(board)

    v := make([][]int, n)
    for k := range v {
        v[k] = make([]int, m)
    }

    var dfs func(x,y,i int) bool
    dfs = func(x,y,i int) bool {

        if i == len(word) {
            return true
        }
        // check if we are inside limits
        if (x < 0 || x >= m) || (y < 0 || y >= n) {
            return false
        }
        // if already visited or the value is not what we are looking
        if v[y][x] == 1 || board[y][x] != word[i] {
            return false
        }

        v[y][x] = 1
        res := dfs(x+1, y, i+1) || dfs(x-1, y, i+1) || dfs(x, y+1, i+1) || dfs(x, y-1, i+1)
        v[y][x] = 0

        return res
    }

    for i := 0; i < n; i++ {
        for j := 0; j < m; j++ {
            if board[i][j] == word[0] {
                r := dfs(j, i, 0)
                if r == true {
                    return true
                }
            }
        }
    }

    return false
}
