package leetcode

/*
    649. Dota2 Senate
    -----------------
*/

func predictPartyVictory(senate string) string {
    rs, ds := 0, 0
    leader_r, leader_d := 0, 0
    stack := []byte(senate)

    for k := range senate {
        if senate[k] == 'R' {
            rs += 1
        } else {
            ds += 1
        }
    }

    for rs > 0 && ds > 0 {
        c := stack[0]
        if c == 'R' {
            if leader_d == 0 {
                stack = append(stack, c)
                leader_r += 1
            } else {
                leader_d -= 1
                rs -= 1
            }
        } else {
            if leader_r == 0 {
                stack = append(stack, c)
                leader_d += 1
            } else {
                leader_r -= 1
                ds -= 1
            }
        }
        stack = stack[1:]
    }

    if rs > 0 {
        return "Radiant"
    } else {
        return "Dire"
    }
}
