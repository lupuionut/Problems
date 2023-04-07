package leetcode

/*
    1020. Number of Enclaves
    ------------------------
    Visit each land (1) until we hit margin or water (0), keeping track of number
    of visited land cells (sum).
*/

func numEnclaves(grid [][]int) int {
    count := 0
    mx := len(grid) - 1
    my := len(grid[0]) - 1

    var visit func(a,b int, c,d *int)
    visit = func(x, y int, marginal *int, sum *int) {

        if x == 0 || x == mx || y == 0 || y == my {
            *marginal = 1
            return
        }

        grid[x][y] = -1
        *sum += 1

        if grid[x+1][y] == 1 {
            visit(x+1, y, marginal, sum)
        }
        if grid[x-1][y] == 1 {
            visit(x-1, y, marginal, sum)
        }
        if grid[x][y-1] == 1 {
            visit(x, y-1, marginal, sum)
        }
        if grid[x][y+1] == 1 {
            visit(x, y+1, marginal, sum)
        }
    }

    for i := 1; i < mx; i++ {
        for j := 1; j < my; j++ {
            if grid[i][j] == 1 {
                marginal := new(int)
                sum := new(int)
                *marginal, *sum = 0, 0
                visit(i, j, marginal, sum)
                if *marginal != 1 {
                    count += *sum
                }
            }
        }
    }

    return count
}
