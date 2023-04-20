package leetcode

/*
    93. Restore IP Addresses
    ------------------------
*/

func restoreIpAddresses(s string) []string {
    if len(s) > 12 || len(s) < 4 {
        return nil
    }

    bytesToInt := func(s []byte) int {
        i := 0
        for k := range s {
            i = (i * 10) + (int(s[k]) - 48)
        }
        return i
    }

    min := func(a, b int) int {
        if a < b {
            return a
        }
        return b
    }

    ips := []string{}

    var dfs func(idx int, current string, level int)
    dfs = func(idx int, current string, level int) {
        if level == 4 && idx == len(s) {
            ips = append(ips, current[0:len(current) - 1])
            return
        }
        for i := idx ; i < min(len(s), idx + 3); i++ {
            if i > idx && s[idx] == '0' {
                break
            }
            if bytesToInt([]byte(s[idx:i+1])) <= 255 {
                dfs(i+1, current + string(s[idx:i+1]) + ".", level + 1)
            }
        }
    }

    dfs(0, "", 0)
    return ips
}
