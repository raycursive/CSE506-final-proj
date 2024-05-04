#!/bin/bash
#
mkdir -p csv
mkdir -p out

for i in {1..5}; do bins/benchrunner-glibc -j16 -p --tree bst --testcase put_s > /dev/null; done

for r in {1..5}; do
    for i in 1 2 4 8 16 32 64; do
        /usr/bin/time -f %U,%S,%P,%M,%R,%w bins/benchrunner-$3 -j$i -p --tree $1 --testcase $2 >> out/$1-$2-$3.out.$r 2>&1
        echo "" >> out/$1-$2-$3.out.$r
    done
done

python3 ./scripts/extract.py $1-$2-$3
