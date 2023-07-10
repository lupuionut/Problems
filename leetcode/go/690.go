/*
    690. Employee Importance
    ------------------------
*/

func getImportance(employees []*Employee, id int) int {
    importance := make([]int, 2001);
    subordinates := make([][]int, 2001);

    for i := range employees {
        e := employees[i]
        importance[e.Id] = e.Importance
        subordinates[e.Id] = e.Subordinates
    }

    ans := importance[id]
    queue := subordinates[id]

    for {
        l := len(queue)
        if l == 0 {
            break;
        }
        for i := range queue {
            sub := queue[i]
            ans += importance[sub]
            queue = append(queue, subordinates[sub]...)
        }
        queue = queue[l:]
    }

    return ans
}
