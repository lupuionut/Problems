package leetcode

import (
    "testing"
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