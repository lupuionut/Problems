package leetcode

type Node struct {
	Val bool
	IsLeaf bool
	TopLeft *Node
	TopRight *Node
	BottomLeft *Node
	BottomRight *Node
}

func Construct(grid [][]int) *Node {
    end := len(grid)
    dp := make([][]*Node, len(grid))
    for k := range dp {
        dp[k] = make([]*Node, len(grid))
    }

    // initiate each cell with the corresponding value
    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid); j++ {
            val := true
            if grid[i][j] == 0 {
                val = false
            }
            dp[i][j] = &Node{Val: val, IsLeaf: true}
        }
    }

    /*
    we must combine top left with top right with bottom left with bottom right
    so we must iterate until we have a single row

    |*|*| | |     | | |*|*|    | | | | |    | | | | |
    |*|*| | |     | | |*|*|    | | | | |    | | | | |
    | | | | |     | | | | |    |*|*| | |    | | |*|*|
    | | | | |     | | | | |    |*|*| | |    | | |*|*|

    |*|*|
    |*|*|

    |*|

    */
    for true {
        if end == 1 {
            break
        }
        for i := 0; i < end; i += 2 {
            for j := 0; j < end; j += 2 {
                dp[i/2][j/2] = NodeCombine(dp[i][j], dp[i][j+1], dp[i+1][j], dp[i+1][j+1])
            }
        }
        end = end/2
    }

    return dp[0][0]
}

func NodeCombine (a, b, c, d *Node) *Node {
    sum := boolToInt(a.Val) + boolToInt(b.Val) + boolToInt(c.Val) + boolToInt(d.Val)
    // if nodes we try to combine are not all leafs
    // or they contain different values (if all 0 -> sum = 0, if all 1 -> sum = 4)
    // we return a new node containing all leafs
    // otherwize return a new node without leafs
    if (a.IsLeaf == false || b.IsLeaf == false || c.IsLeaf == false || d.IsLeaf == false) || (sum != 4 && sum != 0) {
        return &Node{Val: false, IsLeaf: false, TopLeft: a, TopRight: b, BottomLeft: c, BottomRight: d}
    }
    val := true
    if sum == 0 {
        val = false
    }
    return &Node{Val: val, IsLeaf: true, TopLeft: nil, TopRight: nil, BottomLeft: nil, BottomRight: nil}
}

func boolToInt(b bool) int {
    if b == true {
        return 1
    }
    return 0
}
