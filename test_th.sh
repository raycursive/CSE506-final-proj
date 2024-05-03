ARGS=$1
mkdir -p results
RES_FILE_PATH=results/$2.txt
DATA_STRUCTURE=$3

for r in {1..3}; do
  echo Round $r;
  for i in 1 4 8 16 32 64; do
        echo Executing for $i threads;
        make test ARGS="$ARGS -p -j$i" >> "$RES_FILE_PATH";
      done
  echo;
  done

python3 -m process_result $RES_FILE_PATH $DATA_STRUCTURE -rd processed