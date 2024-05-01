package main

import (
	"csp/mergesort"
	"csp/partitioning"
	"encoding/binary"
	"flag"
	"fmt"
	"math/rand"
	"os"
)

func main() {
	mergeCommand := flag.NewFlagSet("merge", flag.ExitOnError)
	threshold := mergeCommand.Int("t", 5, "the threshold where it stops splitting and starts sorting")
	split := mergeCommand.Bool("split", false, "if set runs the split version")
	threads := mergeCommand.Int("th", 1, "the number of threads to start")
	verify := mergeCommand.Bool("v", false, "if set will verify results")
	mergeFile := mergeCommand.String("f", "", "the file to load")

	partCommand := flag.NewFlagSet("part", flag.ExitOnError)
	numThreads := partCommand.Int("th", 1, "the number of threads to start")
	hashBits := partCommand.Int("ha", 1, "the number of hashbits to use")
	file := partCommand.String("f", "", "the file to load")
	onlyLoad := partCommand.Bool("ol", false, "if set only runs the file loading")

	if len(os.Args) < 2 {
		fmt.Println("Expected 'merge' or 'part' subcommands")
		os.Exit(1)
	}

	switch os.Args[1] {
	case "merge":
		mergeCommand.Parse(os.Args[2:])
		list := readMergeFile(*mergeFile)
		if *split {
			runSplitMergesort(list, *verify, *threads, *threshold)
			break
		}
		runConcurrentMergesort(list, *verify, *threshold)
	case "part":
		partCommand.Parse(os.Args[2:])
		data := partitioning.ReadFile(*file)
		if *onlyLoad {
			fmt.Println("Only load")
			return
		}
		partitioning.IndependentPartition(*numThreads, *hashBits, data)
	case "generate":
		var l []uint32
		for i := 0; i < (1 << 24); i += 1 {
			l = append(l, uint32(rand.Int31()))
		}
		fmt.Println(len(l))

	default:
		fmt.Printf("Expected 'merge' or 'part' but got '%s'\n", os.Args[1])
		os.Exit(1)
	}

}
func runConcurrentMergesort(l []uint32, verify bool, threshold int) {
	mergesort.ConcurrentMergesort(l, threshold)

	if verify {
		fmt.Println(mergesort.IsSorted(l))
	}
}

func runSplitMergesort(l []uint32, verify bool, threads int, threshold int) {
	list := mergesort.SplitMergesort(l, threads, threshold)

	if verify {
		fmt.Println(mergesort.IsSorted(list))
	}
}

func readMergeFile(name string) []uint32 {
	// Open the file for reading
	file, err := os.Open(name)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// Get the file size
	fileInfo, err := file.Stat()
	if err != nil {
		panic(err)
	}
	fileSize := fileInfo.Size()

	// Read the file content into a buffer
	buf := make([]byte, fileSize)
	_, err = file.Read(buf)
	if err != nil {
		panic(err)
	}

	// Create a slice to store the decoded uint32 values
	data := make([]uint32, fileSize/4) // 4 bytes per uint32

	// Decode the binary data and store it in the slice
	for i := 0; i < len(data); i++ {
		data[i] = binary.LittleEndian.Uint32(buf[i*4:])
	}

	return data
}
