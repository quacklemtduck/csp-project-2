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

	buffers := make([][][]KeyVal, threads)
	var wg sync.WaitGroup
	for i := 0; i < threads; i += 1 {
		wg.Add(1)
		d := data[i*int(chunkSize) : (i+1)*int(chunkSize)]
		go independentRun(d, int(bufferSize), numBuffers, hashbits, &wg, buffers, i)
	}
	wg.Wait()
	fmt.Println(buffers[0][0][0])
}

func independentRun(data []KeyVal, bufferSize int, numBuffers int, hashbits int, wg *sync.WaitGroup, buffers [][][]KeyVal, id int) {
	defer wg.Done()
	for i := 0; i < numBuffers; i += 1 {
		buffers[id] = append(buffers[id], []KeyVal{})
	}
	for i := range buffers[id] {
		buffers[id][i] = make([]KeyVal, 0, bufferSize)
	}

	for _, kv := range data {
		h := hash(kv.Key, hashbits)
		buffers[id][h] = append(buffers[id][h], kv)
	}
}

func hash(key uint64, hashbits int) uint64 {
	return key % (1 << hashbits)
}

func ReadFile(path string) (data []KeyVal) {
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()
	fileInfo, err := file.Stat()
	if err != nil {
		panic(err)
	}
	fileSize := fileInfo.Size()
	content := make([]byte, fileSize)
	_, err = file.Read(content)
	if err != nil {
		panic(err)
	}

	// content, err := os.ReadFile(path)
	// if err != nil {
	// 	fmt.Println("Error opening file", err)
	// 	os.Exit(1)
	// }

	if (len(content) % 16) != 0 {
		fmt.Println("The file length is not correct, over by", len(content)%16)
		os.Exit(1)
	}

	bigStuff := make([]uint64, len(content)/8)
	for i := 0; i < len(content); i += 8 {
		var val uint64
		for j := 0; j < 8; j++ {
			val |= uint64(content[i+j]) << (uint(j) * 8)
		}
		//bigStuff = append(bigStuff, val)
		bigStuff[i/8] = val
	}

	data = make([]KeyVal, len(bigStuff)/2)

	for i := 0; i < len(bigStuff); i += 2 {
		data = append(data, KeyVal{Key: bigStuff[i], Val: bigStuff[i+1]})
		data[i/2] = KeyVal{Key: bigStuff[i], Val: bigStuff[i+1]}
	}

	return data
}
