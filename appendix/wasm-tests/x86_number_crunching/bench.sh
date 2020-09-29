#!/bin/bash
cargo build --release
starts=(1 4294967295 1007198254740991)
sizes=(1 10 100 1000 10000 100000 1000000 10000000 100000000 1000000000)

for a in ${starts[@]}; do echo "Single Threaded x86 with starts = ${a}"; for x in ${sizes[@]}; do  echo "$x `echo "${a} $((a+x)) 7" | ./target/release/x86_number_crunching` $a $((a+x))"; done; echo "" ; done


#for a in ${starts[@]}; do echo "Multi Threaded x86 with starts = ${a}"; for x in ${sizes[@]}; do  echo "$x `echo "${a} $((a+x)) 7" | ./target/release/x86_number_crunching` $a $((a+x))"; done; echo "" ; done

