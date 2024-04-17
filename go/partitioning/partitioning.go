package partitioning

import (
	"fmt"
	"math"
	"os"
	"sync"
)

type KeyVal struct {
	Key uint64
	Val uint64
}

func IndependentPartition(threads int, hashbits int, data []KeyVal) {
	fmt.Printf("Threads: %v, Hashbits: %v\n", threads, hashbits)
	N := len(data)
	numBuffers := 1 << (hashbits)
	bufferSize := math.Ceil(float64(N) / float64(threads*numBuffers))

	chunkSize := math.Ceil(float64(N) / float64(threads))

	var wg sync.WaitGroup
	for i := 0; i < threads; i += 1 {
		wg.Add(1)
		d := data[i*int(chunkSize) : (i+1)*int(chunkSize)]
		go independentRun(d, int(bufferSize), numBuffers, hashbits, &wg)
	}
	wg.Wait()
}

func independentRun(data []KeyVal, bufferSize int, numBuffers int, hashbits int, wg *sync.WaitGroup) [][]KeyVal {
	defer wg.Done()
	buffers := make([][]KeyVal, numBuffers)
	for i := range buffers {
		buffers[i] = make([]KeyVal, 0, bufferSize)
	}

	for _, kv := range data {
		h := hash(kv.Key, hashbits)
		buffers[h] = append(buffers[h], kv)
	}

	return buffers
}

func hash(key uint64, hashbits int) uint64 {
	return key % (1 << hashbits)
}

func ReadFile(path string) (data []KeyVal) {
	content, err := os.ReadFile(path)
	if err != nil {
		fmt.Println("Error opening file", err)
		os.Exit(1)
	}

	if (len(content) % 16) != 0 {
		fmt.Println("The file length is not correct, over by", len(content)%16)
		os.Exit(1)
	}

	var bigStuff []uint64
	for i := 0; i < len(content); i += 8 {
		var val uint64
		for j := 0; j < 8; j++ {
			val |= uint64(content[i+j]) << (uint(j) * 8)
		}
		bigStuff = append(bigStuff, val)
	}

	for i := 0; i < len(bigStuff); i += 2 {
		data = append(data, KeyVal{Key: bigStuff[i], Val: bigStuff[i+1]})
	}

	return data
}
