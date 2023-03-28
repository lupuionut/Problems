package leetcode

import ("sort")
/*
    18. 4Sum
    -----------------------------------------------------------------
    To use only unique slices, use a slice linked list as a hasmap
    for ex: [1234] => 1 -> 2 -> 3 -> 4
    Trying to insert again [1234] won't work

    Iterate over all values, for each a, b and use c and d as in 2sum case
    [-1 0 1 2 3 4]
    -1 0 [1 .....]
    -1    1[2....]
    -1      2 [3.]
      0 1 [......]
      0     2[...]
*/

func FourSum(nums []int, target int) [][]int {
    if len(nums) < 4 {
        return nil
    }

    sort.Ints(nums)
    n := len(nums) - 1
    results := [][]int{}
    hm := &sliceLinkedList{}

    for i := 0; i < len(nums) - 2; i++ {
        a := nums[i]
        for j := i+1; j < len(nums) - 1; j++ {
            b := nums[j]
            start := j+1
            end := n
            for {
                if start >= end {
                    break
                }
                c := nums[start]
                d := nums[end]
                sum := a + b + c + d
                if sum == target {
                    t := []int{a,b,c,d}
                    if !hm.Contains(t) {
                        results = append(results, t)
                        hm.Insert(t)
                    }
                    start += 1
                } else {
                    if sum > target {
                        end -= 1
                    } else {
                        start += 1
                    }
                }
            }
        }
    }

    return results
}

type sliceLinkedList struct {
    Val int
    Childrens []*sliceLinkedList
}

func (m *sliceLinkedList) Insert(xs []int) {
    if len(xs) == 0 {
        return
    }

    var node *sliceLinkedList

    for k := range m.Childrens {
        if m.Childrens[k].Val == xs[0] {
            node = m.Childrens[k]
            break
        }
    }

    if node == nil {
        child := &sliceLinkedList{Val: xs[0]}
        m.Childrens = append(m.Childrens, child)
        child.Insert(xs[1:])
    } else {
        node.Insert(xs[1:])
    }
}

func (m *sliceLinkedList) Contains(xs []int) bool {
    if len(xs) == 0 {
        return true
    }
    for k := range m.Childrens {
        if m.Childrens[k].Val == xs[0] {
            return m.Childrens[k].Contains(xs[1:])
        }
    }

    return false
}
