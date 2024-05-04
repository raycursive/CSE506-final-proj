# Call format: ./test_th.sh "--tree <tree> --testcase <test_case>" <result_file_name> <data_structure_name>
# data_structure_name - name to show in plots
# result_file_name - name of the output file (without extension)
ARGS=$1
mkdir -p results
RES_FILE_NAME=results/$2.txt
DATA_STRUCTURE=$3

# for r in {1..3}; do
#   echo Round $r;
#   for i in 1 4 8 16 32 64; do
#         echo Executing for $i threads;
#         make test ARGS="$ARGS -p -j$i" >> "$RES_FILE_NAME";
#       done
#   echo;
#   done

python3 -m process_result "$RES_FILE_NAME" "$DATA_STRUCTURE" -rd processed