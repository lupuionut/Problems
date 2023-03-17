package leetcode

type Trie struct {
    Children map[byte]*Trie
    Complete bool
}

func TrieConstructor() Trie {
    children := make(map[byte]*Trie)
    return Trie{Children: children}
}

func (this *Trie) Insert(word string)  {
    if len(word) == 0 {
        this.Complete = true
        return
    }

    var children *Trie

    if val, ok := this.Children[word[0]]; ok {
        children = val
    } else {
        node := TrieConstructor()
        children = &node
        this.Children[word[0]] = children
    }
    children.Insert(word[1:])
}

func (this *Trie) Search(word string) bool {
    if len(word) == 0 {
        if this.Complete == true {
            return true
        }
        return false
    }

    if children, ok := this.Children[word[0]]; ok {
        return children.Search(word[1:])
    }

    return false
}

func (this *Trie) StartsWith(prefix string) bool {
    if len(prefix) == 0 {
        return true
    }

    if children, ok := this.Children[prefix[0]]; ok {
        return children.StartsWith(prefix[1:])
    }

    return false
}
