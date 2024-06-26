cargo b --release

repeats=20
sort_data=../merge.data
part_data=../partition.data
output_dir=../results/

perf stat -r $repeats -j -o $output_dir/rust-part-load.txt -e duration_time,cycles,instructions,context-switches,L1-icache-load-misses,L1-dcache-load-misses,LLC-load-misses,cache-misses,uops_retired.stall_cycles,branch-misses,iTLB-load-misses,dTLB-load-misses ./target/release/seg-and-sort bench-partitioning-data $part_data
perf stat -r $repeats -j -o $output_dir/rust-merge-load.txt -e duration_time,cycles,instructions,context-switches,L1-icache-load-misses,L1-dcache-load-misses,LLC-load-misses,cache-misses,uops_retired.stall_cycles,branch-misses,iTLB-load-misses,dTLB-load-misses ./target/release/seg-and-sort bench-sort-data $sort_data

for num_threads in 1 2 4 8 16 32 64
do
    for num_bits in $(seq 1 18)
    do
        perf stat -r $repeats -j -o $output_dir/rust-part-$num_threads-$num_bits.txt -e duration_time,cycles,instructions,context-switches,L1-icache-load-misses,L1-dcache-load-misses,LLC-load-misses,cache-misses,uops_retired.stall_cycles,branch-misses,iTLB-load-misses,dTLB-load-misses ./target/release/seg-and-sort partitioning $num_threads $num_bits $part_data
    done
done

for num_threads in 1 2 4 8 16 32 64
do
    for threshold in 2 3 4 8 16 32 64 128 256 512 1024
    do
        perf stat -r $repeats -j -o $output_dir/rust-merge-$num_threads-$threshold.txt -e duration_time,cycles,instructions,context-switches,L1-icache-load-misses,L1-dcache-load-misses,LLC-load-misses,cache-misses,uops_retired.stall_cycles,branch-misses,iTLB-load-misses,dTLB-load-misses ./target/release/seg-and-sort mergesort $num_threads $threshold $sort_data
    done
done