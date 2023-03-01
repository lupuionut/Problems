package leetcode

import "fmt"

func FindDuplicateSubtrees(root *TreeNode) []*TreeNode {
	found := []*TreeNode{}
	occurences := make(map[string][]*TreeNode)
	NodeToString(root, occurences)
	for k := range occurences {
		if len(occurences[k]) > 1 {
			found = append(found, occurences[k][0])
		}
	}
    return found
}

func NodeToString(node *TreeNode, occurences map[string][]*TreeNode) string {
	val := ""
	if node == nil {
		return ""
	}
	val += ","
	val += NodeToString(node.Left, occurences)
	val += fmt.Sprint(node.Val)
	val += NodeToString(node.Right, occurences)
	val += ","
	fmt.Println(val)
	if _, ok := occurences[val]; ok {
		occurences[val] = append(occurences[val], node)
	} else {
		occurences[val] = []*TreeNode{node}
	}
	return val
}
