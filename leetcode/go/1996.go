package leetcode

/*
    1996. The Number of Weak Characters in the Game
    -----------------------------------------------
*/

func numberOfWeakCharacters(properties [][]int) int {
    groups := make(map[int][]int)
    min_attack := 10001
    max_attack := 0
    max_defense := 0
    ans := 0

    for k := range properties {
        attack := properties[k][0]
        defense := properties[k][1]
        if _, ok := groups[attack]; !ok {
            groups[attack] = []int{}
        }
        groups[attack] = append(groups[attack], defense)
        if attack > max_attack {
            max_attack = attack
        }
        if attack < min_attack {
            min_attack = attack
        }
    }

    for i := max_attack; i >= min_attack; i-- {
        if val, ok := groups[i]; ok {
            new_max_defense := 0
            for k := range val {
                if val[k] < max_defense {
                    ans += 1
                }
                if val[k] > new_max_defense {
                    new_max_defense = val[k]
                }
            }
            if new_max_defense > max_defense {
                max_defense = new_max_defense
            }
        }
    }

    return ans
}
