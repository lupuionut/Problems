package leetcode

type BrowserHistory struct {
    Page string
    Current *BrowserHistory
    Previous *BrowserHistory
    Next *BrowserHistory
}

func BrowserHistoryConstructor(homepage string) BrowserHistory {
    return BrowserHistory{Page: homepage}
}

func (this *BrowserHistory) Visit(url string)  {
    page := &BrowserHistory{Page: url}

    if this.Current == nil {
        this.Next = page
        page.Previous = this
    } else {
        n := this.Current
        page.Previous = n
        n.Next = page
    }
    this.Current = page
}

func (this *BrowserHistory) Back(steps int) string {
    if this.Current == nil {
        return this.Page
    }
    current := this.Current
    for true {
        if steps == 0 || current.Previous == nil {
            break
        }
        current = current.Previous
        steps -= 1
    }
    this.Current = current
    return current.Page
}

func (this *BrowserHistory) Forward(steps int) string {
    if this.Current == nil {
        return this.Page
    }
    current := this.Current
    for true {
        if steps == 0 || current.Next == nil {
            break
        }
        current = current.Next
        steps -= 1
    }
    this.Current = current
    return current.Page
}
