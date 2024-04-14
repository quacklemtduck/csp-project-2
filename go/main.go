package main

import (
	"fmt"
	"math/rand"
	"os"
	"slices"
	"sync"
)

const THRESHOLD = 5

func main() {

	var l SortableList
	for i := 0; i < (2 << 24); i += 1 {
		l = append(l, uint32(rand.Int31()))
	}

	ConcurrentMergesort(l)

	// Check is it is sorted
	for i := 1; i < len(l); i += 1 {
		if l[i-1] > l[i] {
			fmt.Println("Not sorted")
			os.Exit(1)
		}
	}
	fmt.Println("Sorted")
}

type SortableList []uint32

func ConcurrentMergesort(list SortableList) {
	// if we have too few values we don't split
	if len(list) <= THRESHOLD {
		slices.Sort(list)
		return
	}

	splitIndex := len(list) / 2

	var wg sync.WaitGroup
	wg.Add(2)
	go mergesort(&wg, list[:splitIndex])
	go mergesort(&wg, list[splitIndex:])
	wg.Wait()

	// Merge
	merge(list, splitIndex)
}

func mergesort(parentWg *sync.WaitGroup, list SortableList) {
	defer parentWg.Done()
	// if we have too few values we don't split
	if len(list) <= THRESHOLD {
		slices.Sort(list)
		return
	}

	splitIndex := len(list) / 2

	var wg sync.WaitGroup
	wg.Add(2)
	go mergesort(&wg, list[:splitIndex])
	go mergesort(&wg, list[splitIndex:])
	wg.Wait()

	merge(list, splitIndex)
}

func merge(list SortableList, splitIndex int) {
	i := 0
	arr1 := list[:splitIndex]

	j := 0
	arr2 := list[splitIndex:]

	var dst SortableList

	// Comparing the top element of each list and appending the smallest to dst
	for i < len(arr1) && j < len(arr2) {
		if arr1[i] < arr2[j] {
			dst = append(dst, arr1[i])
			i += 1
		} else {
			dst = append(dst, arr2[j])
			j += 1
		}
	}

	// Copy the rest remaining elements
	if i < len(arr1) {
		dst = append(dst, arr1[i:]...)
	}
	if j < len(arr2) {
		dst = append(dst, arr2[j:]...)
	}

	// Copy the merged list into the original
	copy(list, dst)
}
