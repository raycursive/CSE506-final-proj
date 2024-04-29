import argparse
import os
from collections import defaultdict

# Index of values in the metrics result line
METRIC_IND, MALLOC_IND, THREADS_IND, AVG_THROUGHPUT_IND = 0, 1, 2, -1


def get_value(inp):
    return inp.split(':')[1].strip()


def get_output_file_path(input_file_path, result_dir):
    input_file_name = input_file_path.split('/')[-1]
    res_file_path = os.path.join(result_dir, f"{input_file_name.split('.')[0]}_res.csv")

    return res_file_path


def compute_avg_per_thread_perf(input_file_path, result_dir):
    thread_metric_map = defaultdict(lambda: defaultdict(list))

    with open(input_file_path) as handle:
        for line in handle:
            if 'metric' not in line:
                continue

            tokens = line.split(',')
            metric_name = get_value(tokens[METRIC_IND])

            if not metric_name.startswith('put'):
                continue

            malloc_name = get_value(tokens[MALLOC_IND])
            thread_count = int(get_value(tokens[THREADS_IND]))
            avg_throughput = float(get_value(tokens[AVG_THROUGHPUT_IND]))
            thread_metric_map[thread_count][malloc_name].append(avg_throughput)

    if not thread_metric_map:
        raise ValueError("The provided input file is invalid. File may be empty or result is in wrong format")

    mallocs = next(iter(thread_metric_map.values())).keys()
    res = [['threads', *mallocs]]

    # for threads, mallocs_map in thread_metric_map.items():
    for threads in sorted(thread_metric_map.keys()):
        mallocs_map = thread_metric_map[threads]
        print(f"{threads} threads samples count:", len(mallocs_map[list(mallocs_map.keys())[0]]))
        row = [threads]

        for name in mallocs:
            avg = round(sum(mallocs_map[name]) / len(mallocs_map[name]))
            row.append(avg)

        res.append(row)

    os.makedirs(result_dir, exist_ok=True)
    out_file_path = get_output_file_path(input_file_path, result_dir)

    # Write as CSV
    with open(out_file_path, 'w') as handle:
        for row in res:
            handle.write(','.join(map(str, row)) + '\n')


def parse_args():
    parser = argparse.ArgumentParser()

    parser.add_argument(
        dest="input_file_path",
        action="store",
    )

    parser.add_argument(
        "-rd",
        "--result_dir",
        dest="result_dir",
        action="store",
        default='.'
    )

    return parser.parse_args()


if __name__ == "__main__":
    args = parse_args()
    print("Processing result file:", args.input_file_path)
    compute_avg_per_thread_perf(args.input_file_path, args.result_dir)
