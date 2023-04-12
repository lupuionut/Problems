package leetcode

/*
    71. Simplify Path
    -----------------
*/

func simplifyPath(path string) string {
    dirs := []string{}
    prev, spath := "", ""

    dropLast := func() {
        if len(dirs) == 0 {
            return
        }
        last := len(dirs) - 1
        dirs = dirs[0:last]
    }

    // traverse the path and check for current against previous buffer
    for k := range path {
        current := string(path[k])
        if len(prev) == 0 {
            prev = current
        } else {
            if prev != "/" && prev != "." && prev != ".." {
                if current == "/" {
                    dirs = append(dirs, prev)
                    prev = "/"
                    continue
                } else {
                    prev += current
                }
            }
            if prev == "/" {
                if current == "/" {
                    continue
                }
                if current == "." {
                    prev = "."
                } else {
                    prev = current
                }
                continue
            }
            if prev == "." {
                if current == "/" {
                    prev = "/"
                    continue
                }
                if current == "." {
                    prev = ".."
                } else {
                    prev += current
                }
                continue
            }
            if prev == ".." {
                if current == "/" {
                    dropLast()
                    prev = "/"
                } else {
                    prev += current
                }
            }
        }
    }

    // check what remain in previous buffer when we finished reading the path
    if prev == ".." {
        dropLast()
    } else {
        if prev != "/" && prev != "." {
            dirs = append(dirs, prev)
        }
    }

    if len(dirs) == 0 {
        return "/"
    }

    for k := range dirs {
        spath += "/" + dirs[k]
    }

    return spath
}
