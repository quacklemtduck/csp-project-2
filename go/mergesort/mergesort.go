package mergesort

import (
	"fmt"
	"slices"
	"sync"
)

type SortableList []uint32

var THRESHOLD = 5

func ConcurrentMergesort(list SortableList, threshold int) {
	THRESHOLD = threshold

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

func SplitMergesort(list SortableList, numThreads int) SortableList {
	lists := splitSlice(list, numThreads)

	var wg sync.WaitGroup

	for _, l := range lists {
		wg.Add(1)
		go func() {
			sequentialMergesort(l)
			wg.Done()
		}()
	}
	wg.Wait()

	list = mergeParts(lists)
	return list
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

func sequentialMergesort(list SortableList) {
	// if we have too few values we don't split
	if len(list) <= THRESHOLD {
		slices.Sort(list)
		return
	}

	splitIndex := len(list) / 2

	sequentialMergesort(list[:splitIndex])
	sequentialMergesort(list[splitIndex:])

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

func splitSlice(slice SortableList, parts int) []SortableList {
	if parts <= 0 {
		return nil
	}

	// Calculate the size of each part.
	size := (len(slice) + parts - 1) / parts

	result := make([]SortableList, 0, parts)
	for i := 0; i < len(slice); i += size {
		end := i + size
		if end > len(slice) {
			end = len(slice)
		}
		result = append(result, slice[i:end])
	}

	return result
}

func IsSorted(slice SortableList) bool {
	for i := 1; i < len(slice); i++ {
		if slice[i-1] > slice[i] {
			fmt.Printf("%v and %v\n at %v\n", slice[i-1], slice[i], i)
			return false
		}
	}
	return true
}

// merge merges two sorted slices into a single sorted slice.
func splitMerge(left, right SortableList) SortableList {
	merged := make(SortableList, 0, len(left)+len(right))
	for len(left) > 0 || len(right) > 0 {
		if len(left) == 0 {
			return append(merged, right...)
		}
		if len(right) == 0 {
			return append(merged, left...)
		}
		if left[0] <= right[0] {
			merged = append(merged, left[0])
			left = left[1:]
		} else {
			merged = append(merged, right[0])
			right = right[1:]
		}
	}
	return merged
}

// mergeParts merges the divided parts into a single slice.
func mergeParts(parts []SortableList) SortableList {
	if len(parts) == 1 {
		return parts[0]
	}

	mid := len(parts) / 2
	left := mergeParts(parts[:mid])
	right := mergeParts(parts[mid:])
	return splitMerge(left, right)
}
