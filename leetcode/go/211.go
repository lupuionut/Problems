package leetcode

type WordDictionary struct {
    Next map[byte]*WordDictionary
    EndOfWord bool
}

func WordDictionaryConstructor() WordDictionary {
    next := make(map[byte]*WordDictionary)
    return WordDictionary{Next: next}
}

func (this *WordDictionary) AddWord(word string) {
    if len(word) == 0 {
        this.EndOfWord = true
        return
    }

    if dict, ok := this.Next[word[0]]; ok {
        dict.AddWord(word[1:])
    } else {
        node := WordDictionaryConstructor()
        dict := &node
        this.Next[word[0]] = dict
        dict.AddWord(word[1:])
    }
}

func (this *WordDictionary) Search(word string) bool {
    if len(word) == 0 && this.EndOfWord == true {
        return true
    }
    if len(word) == 0 {
        return false
    }

    result := false
    if word[0] == '.' {
        for k := range this.Next {
            result = result || this.Next[k].Search(word[1:])
            if result == true {
                break
            }
        }
        return result
    }

    if dict, ok := this.Next[word[0]]; ok {
        return dict.Search(word[1:])
    }

    return false
}
