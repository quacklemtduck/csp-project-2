package main

import (
	"csp/mergesort"
	"csp/partitioning"
	"flag"
	"fmt"
	"math/rand"
	"os"
)

func main() {
	mergeCommand := flag.NewFlagSet("merge", flag.ExitOnError)
	threshold := mergeCommand.Int("t", 5, "the threshold where it stops splitting and starts sorting")

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
		runMergesort(*threshold)
	case "part":
		partCommand.Parse(os.Args[2:])
		data := partitioning.ReadFile(*file)
		if *onlyLoad {
			fmt.Println("Only load")
			return
		}
		partitioning.IndependentPartition(*numThreads, *hashBits, data)
	default:
		fmt.Printf("Expected 'merge' or 'part' but got '%s'\n", os.Args[1])
		os.Exit(1)
	}

}
func runMergesort(threshold int) {
	var l []uint32
	for i := 0; i < (1 << 24); i += 1 {
		l = append(l, uint32(rand.Int31()))
	}

	mergesort.ConcurrentMergesort(l, threshold)

}
