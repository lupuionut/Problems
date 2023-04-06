package leetcode

/*
    1254. Number of Closed Islands
    ------------------------------
    Each time we find a 0 (land), visit in a recursive way each connected land until
    we either hit the margin(so it's not an island) or we hit the water (1).
    Mark each visited cell by making it -1 so that we don't visit again.
*/

func closedIsland(grid [][]int) int {
    count := 0
    mx := len(grid) - 1
    my := len(grid[0]) - 1

    var visit func(a,b int, c *int)

    visit = func(x, y int, marginal *int) {

        if x == 0 || x == mx || y == 0 || y == my {
            *marginal = 1
            return
        }

        grid[x][y] = -1

        if grid[x+1][y] == 0 {
            visit(x+1, y, marginal)
        }
        if grid[x-1][y] == 0 {
            visit(x-1, y, marginal)
        }
        if grid[x][y-1] == 0 {
            visit(x, y-1, marginal)
        }
        if grid[x][y+1] == 0 {
            visit(x, y+1, marginal)
        }
    }

    for i := 1; i < mx; i++ {
        for j := 1; j < my; j++ {
            if grid[i][j] == 0 {
                marginal := new(int)
                *marginal = 0
                visit(i, j, marginal)
                if *marginal != 1 {
                    count += 1
                }
            }
        }
    }

    return count
}
