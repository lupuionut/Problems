package leetcode

/*
    50. Pow(x, n)
    -------------
    Reduce the problem x^n = x^(n/2) * x^(n/2)
    Use cache to increase speed.
*/

func myPow(x float64, n int) float64 {
    cache := make(map[int]float64)
    return ppow(x, n, cache)
}

func ppow(x float64, n int, cache map[int]float64) float64 {
    if n == 0 {
        return float64(1)
    }
    if n == 1 {
        return x
    }

    if val, ok := cache[n]; ok {
        return val
    }

    if n < 0 {
        cache[n] = 1/(ppow(x, 0-n, cache))
        return cache[n]
    }

    f := n/2
    s := f
    if n%2 != 0 {
        s += 1
    }

    cache[n] = ppow(x, f, cache) * ppow(x, s, cache)
    return cache[n]
}
