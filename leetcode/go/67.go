package leetcode

func AddBinary(a string, b string) string {
    var result string
    max := len(a)
    if len(b) > max {
        max = len(b)
    }
    a = pad(a, max)
    b = pad(b, max)
    c := new(string)
    *c = "0"

    for k := max - 1; k >= 0; k-- {
        r := addition(string(a[k]), string(b[k]), c)
        result = r + result
    }

    if (*c == "1") {
        result = "1" + result
    }

    return result
}

func addition(a string, b string, c *string) string {
    e := a + b + *c
    if e == "010" || e == "001" || e == "100"{
        *c = "0"
        return "1"   
    }
    if e == "011" || e == "110" || e == "101" {
        *c = "1"
        return "0"
    }
    if e == "111" {
        *c = "1"
        return "1"
    }
    *c = "0"
    return "0"
}

func pad(s string, max int) string {
    if len(s) == max {
        return s
    }
    delta := max - len(s)
    for k := 0; k < delta; k++ {
        s = "0" + s
    }

    return s
}