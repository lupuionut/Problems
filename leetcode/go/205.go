package leetcode

/*
    205. Isomorphic Strings
    -----------------------
    Keep corresponding chars from s to t and from t to s
    into 2 maps and check for each char.
*/

func isIsomorphic(s string, t string) bool {

    s_t := make(map[byte]byte)
    t_s := make(map[byte]byte)

    for k := range s {
       if val, ok := s_t[s[k]]; ok {
           if val != t[k] {
               return false
           }
       } else {
           if _, ok := t_s[t[k]]; ok {
               return false
           } else {
               s_t[s[k]] = t[k]
               t_s[t[k]] = s[k]
           }
       }
    }

    return true
}
