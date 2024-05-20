package main

import (
	"encoding/binary"
	"flag"
	"fmt"
	"math/rand"
	"os"
)

func main() {
	method := flag.String("m", "none", "the method to generate data for, either merge or partition")
	filename := flag.String("f", "output.bin", "specify the filename to use")
	size := flag.Int64("s", 1<<24, "specify the size of input, default is 1<<24")

	if len(os.Args) < 2 {
		panic("You need to supply a method")
	}
	flag.Parse()
	switch *method {
	case "merge":
		generateMergeData(*filename, *size)
	case "partition":
		generatePartitionData(*filename, uint64(*size))
	default:
		fmt.Printf("Expected 'merge' or 'partition' got %v", *method)
		os.Exit(1)

	}
}

func generateMergeData(filename string, size int64) {
	fmt.Printf("Generating merge data of size %v...\n", size)
	var data []uint32
	for i := 0; i < int(size); i += 1 {
		data = append(data, uint32(rand.Int31()))
	}

	fmt.Printf("Writing results to file %v...\n", filename)
	file, err := os.Create(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// Create a buffer to store binary data
	buf := make([]byte, 4*len(data)) // 4 bytes per uint32

	// Encode []uint32 data to binary and write to the file
	for i, v := range data {
		binary.LittleEndian.PutUint32(buf[i*4:], v)
	}

	_, err = file.Write(buf)
	if err != nil {
		panic(err)
	}

	fmt.Println("Done!")
}

func generatePartitionData(filename string, size uint64) {
	fmt.Printf("Generating partition data of size %v...\n", size)
	var data []uint64
	var i uint64
	for i = 0; i < size; i += 1 {
		data = append(data, i)
		data = append(data, uint64(rand.Int63()))
	}

	fmt.Printf("Writing results to file %v...\n", filename)
	file, err := os.Create(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// Create a buffer to store binary data
	buf := make([]byte, 8*len(data)) // 4 bytes per uint32

	// Encode []uint32 data to binary and write to the file
	for i, v := range data {
		binary.LittleEndian.PutUint64(buf[i*8:], v)
	}

	_, err = file.Write(buf)
	if err != nil {
		panic(err)
	}

	fmt.Println("Done!")

}
