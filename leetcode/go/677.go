package leetcode

/*
    677. Map Sum Pairs
    ------------------
    Use a trie like structure and keep track of total for each node.
    When we insert, search if already inserted, get its value and
    insert with corrected value (new value - existing value).
    Each ending node will contain the total value and the value for
    the corresponding ending word.
*/

type MapSum struct {
    Leafs []*MapSum
    Total int
    Value int
    Key byte
}

func MapSumConstructor() MapSum {
    return MapSum{}
}

func (this *MapSum) Insert(key string, val int)  {

    existent := this.Get(key)
    corrected := val
    if existent != nil {
        corrected -= existent.Value
    }

    node := this
    i := 0
    loop:
    for {
        if i == len(key) {
            node.Value = val
            break
        }

        for k := range node.Leafs {
            if node.Leafs[k].Key == key[i] {
                node = node.Leafs[k]
                node.Total += corrected
                i += 1
                continue loop
            }
        }

        nnode := &MapSum{Key: key[i], Total: corrected}
        node.Leafs = append(node.Leafs, nnode)
        node = nnode
        i += 1
    }
}

func (this *MapSum) Sum(prefix string) int {
    existent := this.Get(prefix)
    if existent == nil {
        return 0
    }
    return existent.Total
}

func (this *MapSum) Get(key string) *MapSum {
    node := this
    i := 0
    loop:
    for {
        if i == len(key) {
            return node
        }

        for k := range node.Leafs {
            if node.Leafs[k].Key == key[i] {
                node = node.Leafs[k]
                i += 1
                continue loop
            }
        }

        break
    }

    return nil
}
