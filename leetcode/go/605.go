package leetcode

func canPlaceFlowers(flowerbed []int, n int) bool {

    if n == 0 {
        return true
    }

    previous := 0
    for i := 0; i < len(flowerbed); i++ {
        if i == len(flowerbed) - 1 {
            if flowerbed[i] == 0 && previous == 0 {
                n -= 1
            }
            break
        }

        if flowerbed[i] == 0 && previous == 0 && flowerbed[i+1] == 0 {
            n -= 1
            previous = 1
        } else {
            previous = flowerbed[i]
        }

        if n == 0 {
            break
        }
    }

    if n == 0 {
        return true
    }

    return false
}
