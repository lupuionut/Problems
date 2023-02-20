package leetcode

import (
    "testing"
    "fmt"
)

type Sample struct {
    Input interface{}
    Expected interface{}
}

func Test_45 (t *testing.T) {
    samples := []*Sample{ 
        {Input: []int{1,2,3}, Expected: 2,},
        {Input: []int{2,3,0,1,4}, Expected: 2,},
        {Input: []int{2,3,1,1,4}, Expected: 2,},
        {Input: []int{0}, Expected: 0,},
        {Input: []int{1}, Expected: 0,},
        {Input: []int{3,2,1}, Expected: 1,},
        {Input: []int{2,3,1}, Expected: 1,},
        {Input: []int{1,2,1,1,1}, Expected: 3,},
        {Input: []int{1,2}, Expected: 1,},
        {Input: []int{4,1,1,3,1,1,1}, Expected: 2,},
        {Input: []int{5,9,3,2,1,0,2,3,3,1,0,0}, Expected: 3,},
        {Input: []int{10,9,8,7,6,5,4,3,2,1,1,0}, Expected: 2,},
        {Input: []int{2,2,0,1}, Expected: 2,},
}
    for _, sample := range samples {
        result := Jump(sample.Input.([]int))
        expected := sample.Expected.(int)
        if result != expected {
            t.Errorf("FAIL: For sample %v expected result %d, but got %d", sample.Input.([]int), expected, result)
        } else {
            t.Logf("PASS: For sample %v expected result %d and we got %d", sample.Input.([]int), expected, result)
        }
    }
}

func Test_1162 (t *testing.T) {
    samples := []*Sample{
        {Input: [][]int{{1,0,1}, {0,0,0}, {1,0,1}}, Expected: 2,},
        {Input: [][]int{{1,0,0}, {0,0,0}, {0,0,0}}, Expected: 4,},
        {Input: [][]int{{1,0,0}, {0,0,0}}, Expected: 3,},
        {Input: [][]int{{0,0,0}, {0,0,0}}, Expected: -1,},
        {Input: [][]int{{0,0,1}}, Expected: 2,},
    }
    for k, sample := range samples {
        result := MaxDistance(sample.Input.([][]int))
        expected := sample.Expected.(int)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %d, but got %d", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %d and we got %d", k, expected, result)
        }
    }
}

func Test_2306 (t *testing.T) {
    samples := []*Sample{
        {Input: []string{"a", "b"}, Expected: 0,},
        {Input: []string{"lack", "back"}, Expected: 0,},
        {Input: []string{"coffee", "cocos", "hello", "donuts", "dare"}, Expected: 16},
        {Input: []string{"coffee","donuts","time","toffee"}, Expected: 6,},
        {Input: []string{"aaa","baa","caa","bbb","cbb","dbb"}, Expected: 2,},
        {Input: []string{"coffee","donuts","time","toffee", "teer", "twin"}, Expected: 10,},
        {Input: []string{"sfuzder","spklurny","kvdolme","kbbdklkpgk","za","mdbsmnjiu","pzydbfwiw","lvvyshmd","ow","ssipb","jucpsquexm","ffuzder","uftruik","ringlxh","jz","sjlxouiv","csdbtsvg","openygbaix","dcn","r","hwql","ok","y","sze","ttptd","atxp","ejfpsea","vjfpsea","lj","drmvufbt","zxambug","ozpxq","qzydbfwiw","lqxik","bjo","rrmvufbt","jvl","rzxaaa","nmfydhawa","shlwkf","rcl","hhn","yrmcsc","yieuzizaeh","nrmvufbt","rinsldgdpp","wqg","tyhkgqcu","rsdbtsvg","zvehtqiqxa","jtz","mjaguebiy","fztbpozhf","zzpxq","ue","foktqxiqe","ztf","dn","lxambug","kinsldgdpp","jhn","k","i","qxtava","roktqxiqe","hr","bwzw","mot","cadj","x","bcep","u","kzydbfwiw","ahku","ijo"}, Expected: 4934},
    }
    for k, sample := range samples {
        result := DistinctNames(sample.Input.([]string))
        expected := sample.Expected.(int)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %d, but got %d", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %d and we got %d", k, expected, result)
        }
    }
}

func Test_2477 (t *testing.T) {
    type Params struct {
        First [][]int
        Second int
    }
    samples := []*Sample{
        {Input: &Params{First: [][]int{{1,0},{0,2},{3,1},{1,4},{5,0}}, Second: 1,}, Expected: 7,},
        {Input: &Params{First: [][]int{{3,1},{3,2},{1,0},{0,4},{0,5},{4,6}}, Second: 2}, Expected: 7,},
        {Input: &Params{First: [][]int{{0,1},{1,2},{1,3},{4,2},{5,3},{6,3},{6,7},{8,6},{9,0},{5,10},{11,9},{12,5},{5,13},{8,14},{11,15},{8,16},{17,0},{18,7}}, Second: 13}, Expected: 19},
        {Input: &Params{First: [][]int{}, Second: 1}, Expected: 0,},
        {Input: &Params{First: [][]int{{0,1}, {0,2}, {0,3}}, Second: 5}, Expected: 3,},
    }

    for k, sample := range samples {
        result := MinimumFuelCost(sample.Input.(*Params).First, sample.Input.(*Params).Second)
        expected := sample.Expected.(int)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %d, but got %d", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %d and we got %d", k, expected, result)
        }
    }
}

func Test_989 (t *testing.T) {
    type Params struct {
        First []int
        Second int
    }
    samples := []*Sample{
        {Input: &Params{First: []int{0}, Second: 0,}, Expected: "[0]",},
        {Input: &Params{First: []int{9}, Second: 1,}, Expected: "[1 0]",},
        {Input: &Params{First: []int{1,2,0,0}, Second: 34,}, Expected: "[1 2 3 4]",},
        {Input: &Params{First: []int{2,7,4}, Second: 181,}, Expected: "[4 5 5]",},
        {Input: &Params{First: []int{2,1,5}, Second: 806,}, Expected: "[1 0 2 1]",},
        {Input: &Params{First: []int{1,2,6,3,0,7,1,7,1,9,7,5,6,6,4,4,0,0,6,3}, Second: 516,}, Expected: "[1 2 6 3 0 7 1 7 1 9 7 5 6 6 4 4 0 5 7 9]",},
    }

    for k, sample := range samples {
        result := AddToArrayForm(sample.Input.(*Params).First, sample.Input.(*Params).Second)
        expected := sample.Expected.(string)
        if toString(result) != expected {
            t.Errorf("FAIL: For sample %d expected result %s, but got %s", k, expected, toString(result))
        } else {
            t.Logf("PASS: For sample %d expected result %s and we got %s", k, expected, toString(result))
        }
    }
}

func toString[T any](variable T) string {
    return fmt.Sprintf("%v", variable)
}

func Test_67 (t *testing.T) {
    type Params struct {
        First string
        Second string
    }
    samples := []*Sample{
        {Input: &Params{First: "1", Second: "1",}, Expected: "10",},
        {Input: &Params{First: "11", Second: "1",}, Expected: "100",},
        {Input: &Params{First: "1010", Second: "1011",}, Expected: "10101",},
        {Input: &Params{First: "0001", Second: "00110",}, Expected: "00111",},
    }

    for k, sample := range samples {
        result := AddBinary(sample.Input.(*Params).First, sample.Input.(*Params).Second)
        expected := sample.Expected.(string)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %s, but got %s", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %s and we got %s", k, expected, result)
        }
    }
}

func Test_1523 (t *testing.T) {
    type Params struct {
        First int
        Second int
    }
    samples := []*Sample{
        {Input: &Params{First: 0, Second: 0,}, Expected: 0,},
        {Input: &Params{First: 0, Second: 11,}, Expected: 6,},
        {Input: &Params{First: 0, Second: 8,}, Expected: 4,},
        {Input: &Params{First: 10, Second: 11,}, Expected: 1,},
        {Input: &Params{First: 0, Second: 3,}, Expected: 2,},
        {Input: &Params{First: 1, Second: 3,}, Expected: 2,},
        {Input: &Params{First: 3, Second: 7,}, Expected: 3,},
        {Input: &Params{First: 8, Second: 10,}, Expected: 1,},
    }

    for k, sample := range samples {
        result := CountOdds(sample.Input.(*Params).First, sample.Input.(*Params).Second)
        expected := sample.Expected.(int)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %d, but got %d", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %d and we got %d", k, expected, result)
        }
    }
}

func Test_1017 (t *testing.T) {
    type Params struct {
        First string
        Second string
    }
    samples := []*Sample{
        {Input: &Params{First:  "TAUXXTAUXXTAUXXTAUXXTAUXX", Second: "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX",}, Expected: "TAUXX",},
        {Input: &Params{First: "ABCABC", Second: "ABC",}, Expected: "ABC",},
        {Input: &Params{First: "ABABAB", Second: "ABAB",}, Expected: "AB",},
        {Input: &Params{First: "A", Second: "A",}, Expected: "A",},
        {Input: &Params{First: "A", Second: "B",}, Expected: "",},
        {Input: &Params{First: "BBB", Second: "BB",}, Expected: "B",},
        {Input: &Params{First: "BBBA", Second: "BB",}, Expected: "",},
        {Input: &Params{First: "BBBA", Second: "AABB",}, Expected: "",},
    }

    for k, sample := range samples {
        result := GcdOfStrings(sample.Input.(*Params).First, sample.Input.(*Params).Second)
        expected := sample.Expected.(string)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %s, but got %s", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %s and we got %s", k, expected, result)
        }
    }
}

func Test_104 (t *testing.T) {
    samples := []*Sample{
        {Input: &TreeNode{Val: 3, Left: &TreeNode{Val: 9, Left: nil, Right: nil} , Right: &TreeNode{Val: 20, Left: &TreeNode{Val: 15, Left: nil, Right: nil}, Right: &TreeNode{Val: 7, Right: nil, Left: nil},},}, Expected: 3,},
    }
    for k, sample := range samples {
        result := MaxDepth(sample.Input.(*TreeNode))
        expected := sample.Expected.(int)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %d, but got %d", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %d and we got %d", k, expected, result)
        }
    }
}

func Test_783 (t *testing.T) {
    samples := []*Sample{
        {Input: &TreeNode{Val: 90, Left: &TreeNode{Val: 69, Left: &TreeNode{Val: 49, Left: nil, Right: &TreeNode{Val: 52, Left: nil, Right: nil,},}, Right: &TreeNode{Val: 89, Left: nil, Right: nil,}}, Right: nil}, Expected: 1,},
        {Input: &TreeNode{Val: 4, Left: &TreeNode{Val: 2, Left: &TreeNode{Val: 1, Left: nil, Right: nil,}, Right: &TreeNode{Val: 3, Left: nil, Right: nil,}}, Right: &TreeNode{Val: 6, Left: nil, Right: nil,}}, Expected: 1,},
        {Input: &TreeNode{Val: 1, Left: &TreeNode{Val: 0, Left: nil, Right: nil,}, Right: &TreeNode{Val: 48, Left: &TreeNode{Val: 12, Left: nil, Right: nil,}, Right: &TreeNode{Val: 49, Left: nil, Right: nil,}}}, Expected: 1,},
        {Input: &TreeNode{Val: 0, Left: nil, Right: &TreeNode{Val: 1, Left: nil, Right: nil,}}, Expected: 1,},
    }
    for k, sample := range samples {
        result := MinDiffInBST(sample.Input.(*TreeNode))
        expected := sample.Expected.(int)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %d, but got %d", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %d and we got %d", k, expected, result)
        }
    }
}

func Test_1129 (t *testing.T) {
    type Params struct {
        First int
        Second [][]int
        Third [][]int
    }
    samples := []*Sample{
        {Input: &Params{First: 5, Second: [][]int{{0,1},{1,3},{2,1},{3,1},{2,0},{1,0},{0,2}}, Third: [][]int{{1,2},{3,2},{2,4},{3,3},{0,3},{1,4},{0,1},{0,2},{0,0},{4,1}}}, Expected: "[0 1 1 1 2]",},
        {Input: &Params{First: 5, Second: [][]int{{2,0},{4,3},{4,4},{3,0},{1,4}}, Third: [][]int {{2,1},{4,3},{3,1},{3,0},{1,1},{2,0},{0,3},{3,3},{2,3}}}, Expected: "[0 -1 -1 1 -1]",},
        {Input: &Params{First: 5, Second: [][]int{{0,1}, {1,2}, {2,3}, {3,4}}, Third: [][]int{{1,2}, {2,3}, {3,1}}}, Expected: "[0 1 2 3 7]",},
        {Input: &Params{First: 3, Second: [][]int{{0,1},{1,2}}, Third: [][]int{}}, Expected: "[0 1 -1]",},
        {Input: &Params{First: 5, Second: [][]int{{0,1},{2,3},{2,4}}, Third: [][]int{{1,2},{3,4}}}, Expected: "[0 1 2 3 3]",},
    }

    for k, sample := range samples {
        result := ShortestAlternatingPaths(
            sample.Input.(*Params).First,
            sample.Input.(*Params).Second,
            sample.Input.(*Params).Third,
        )
        expected := sample.Expected.(string)
        if toString(result) != expected {
            t.Errorf("FAIL: For sample %d expected result %s, but got %s", k, toString(expected), toString(result))
        } else {
            t.Logf("PASS: For sample %d expected result %s and we got %s", k, toString(expected), toString(result))
        }
    }
}

func Test_226 (t *testing.T) {
    samples := []*Sample{
        {Input: &TreeNode{Val: 4, Left: &TreeNode{Val: 2, Left: &TreeNode{Val: 1, Left: nil, Right: nil,}, Right: &TreeNode{Val: 3, Left: nil, Right: nil,},}, Right: &TreeNode{Val: 7, Left: &TreeNode{Val: 6, Left: nil, Right: nil,}, Right: &TreeNode{Val: 9, Left: nil, Right: nil,},},}, Expected: &TreeNode{Val: 4, Left: &TreeNode{Val: 7, Left: &TreeNode{Val: 9, Left: nil, Right: nil,}, Right: &TreeNode{Val: 6, Left: nil, Right: nil,},}, Right: &TreeNode{Val: 2, Left: &TreeNode{Val: 3, Left: nil, Right: nil,}, Right: &TreeNode{Val: 1, Left: nil, Right: nil,}}}},
    }
    for k, sample := range samples {
        result := InvertTree(sample.Input.(*TreeNode))
        expected := sample.Expected.(*TreeNode)
        if result.String() != expected.String() {
            t.Errorf("FAIL: For sample %d expected result %s, but got %s", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %s and we got %s", k, expected, result)
        }
    }
}

func Test_953 (t *testing.T) {
    type Params struct {
        First []string
        Second string
    }
    samples := []*Sample{
        {Input: &Params{First: []string{"hello","leetcode"}, Second: "hlabcdefgijkmnopqrstuvwxyz"}, Expected: true,},
        {Input: &Params{First: []string{"word","world","row"}, Second: "worldabcefghijkmnpqstuvxyz"}, Expected: false,},
        {Input: &Params{First: []string{"apple","app"}, Second: "hlabcdefgijkmnopqrstuvwxyz"}, Expected: false,},
    }
    for k, sample := range samples {
        result := IsAlienSorted(sample.Input.(*Params).First, sample.Input.(*Params).Second)
        expected := sample.Expected.(bool)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %v, but got %v", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %v and we got %v", k, expected, result)
        }
    }
}

func Test_103 (t *testing.T) {
    samples := []*Sample{
        {Input: &TreeNode{Val: 3, Left: &TreeNode{Val: 9, Left: nil, Right: nil}, Right: &TreeNode{Val: 20, Left: &TreeNode{Val: 15, Left: nil, Right: nil,}, Right: &TreeNode{Val: 7, Left: nil, Right: nil,}}}, Expected: "[[3] [20 9] [15 7]]",},
        {Input: &TreeNode{Val: 1, Left: nil, Right: nil}, Expected: "[[1]]",},
        {Input: &TreeNode{Val: 0,Left: &TreeNode{Val: 2,Left: &TreeNode{Val: 1,Left: &TreeNode{Val: 5,Left: nil,Right: nil,},Right: &TreeNode{Val: 1,Left: nil,Right: nil,},},Right: nil,},Right: &TreeNode{Val: 4,Left: &TreeNode{Val: 3,Left: nil,Right: &TreeNode{Val: 6,Left: nil,Right: nil,},},Right: &TreeNode{Val: -1,Left: nil,Right: &TreeNode{Val: 8,Left: nil,Right: nil,},},},}, Expected: "[[0] [4 2] [1 3 -1] [8 6 1 5]]"},
    }
    toString := func(xs [][]int) string {
        return fmt.Sprintf("%v", xs)
    }
    for k, sample := range samples {
        result := ZigzagLevelOrder(sample.Input.(*TreeNode))
        expected := sample.Expected.(string)
        if toString(result) != expected {
            t.Errorf("FAIL: For sample %d expected result %s, but got %s", k, expected, toString(result))
        } else {
            t.Logf("PASS: For sample %d expected result %s and we got %s", k, expected, toString(result))
        }
    }
}

func Test_6 (t *testing.T) {
    type Params struct {
        First string
        Second int
    }
    samples := []*Sample{
        {Input: &Params{First: "PAYPALISHIRING", Second: 3,}, Expected: "PAHNAPLSIIGYIR"},
        {Input: &Params{First: "PAYPALISHIRING", Second: 4,}, Expected: "PINALSIGYAHRPI"},
        {Input: &Params{First: "A", Second: 1,}, Expected: "A"},
        {Input: &Params{First: "BA", Second: 1,}, Expected: "BA"},
        {Input: &Params{First: "BAR", Second: 2,}, Expected: "BRA"},
    }
    for k, sample := range samples {
        result := Convert(sample.Input.(*Params).First, sample.Input.(*Params).Second)
        expected := sample.Expected.(string)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %s, but got %s", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %s and we got %s", k, expected, result)
        }
    }
}

func Test_35 (t *testing.T) {
    type Params struct {
        First []int
        Second int
    }
    samples := []*Sample{
        {Input: &Params{First: []int{1,2,3,4}, Second: 2}, Expected: 0,},
        {Input: &Params{First: []int{1,3,5}, Second: 4}, Expected: 2,},
        {Input: &Params{First: []int{1,3,5,6}, Second: 0}, Expected: 0,},
        {Input: &Params{First: []int{1,3,5,6}, Second: 5}, Expected: 2,},
        {Input: &Params{First: []int{1,3,5,6}, Second: 2}, Expected: 1,},
        {Input: &Params{First: []int{1,3,5,6}, Second: 7}, Expected: 4,},
    }
    for k, sample := range samples {
        result := SearchInsert(sample.Input.(*Params).First, sample.Input.(*Params).Second)
        expected := sample.Expected.(int)
        if result != expected {
            t.Errorf("FAIL: For sample %d expected result %d, but got %d", k, expected, result)
        } else {
            t.Logf("PASS: For sample %d expected result %d and we got %d", k, expected, result)
        }
    }
}
