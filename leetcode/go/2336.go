package leetcode

/*
    2336. Smallest Number in Infinite Set
    -------------------------------------
*/

type SmallestInfiniteSet struct {
    Popped []int
}

func SmallestConstructor() SmallestInfiniteSet {
    popped := make([]int, 1001)
    return SmallestInfiniteSet{Popped: popped}
}

func (this *SmallestInfiniteSet) PopSmallest() int {
    for i := 1; i <= 1000; i++ {
        if this.Popped[i] == 0 {
            this.Popped[i] = 1
            return i
        }
    }
    return len(this.Popped)
}

func (this *SmallestInfiniteSet) AddBack(num int)  {
    if num <= 1000 {
        this.Popped[num] = 0
    }
}
