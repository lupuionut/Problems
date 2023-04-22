package leetcode

/*
    75. Sort Colors
    ---------------
    Keep an ending index for each of colours.
    So each time we find a smaller value, the
    greater values endings will increase with 1
    since we are inserting the smaller value before.
*/

func sortColors(nums []int)  {
    ending := [3]int{-1, -1, -1}

    for k := range nums {
        if nums[k] == 0 {
            ending[0] += 1
            nums[ending[0]] = 0
            if ending[1] != -1 {
                ending[1] += 1
                nums[ending[1]] = 1
            }
            if ending[2] != -1 {
                ending[2] += 1
                nums[ending[2]] = 2
            }
            continue
        }
        if nums[k] == 1 {
            if ending[1] == -1 {
                ending[1] = ending[0] + 1
            } else {
                ending[1] += 1
            }
            nums[ending[1]] = 1
            if ending[2] != -1 {
                ending[2] += 1
                nums[ending[2]] = 2
            }
            continue
        }
        if nums[k] == 2 {
            if ending[2] == -1 {
                if ending[1] == -1 {
                    ending[2] = ending[0] + 1
                } else {
                    ending[2] = ending[1] + 1
                }
            } else {
                ending[2] += 1
            }
            continue
        }
    }
}
