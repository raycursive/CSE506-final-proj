import argparse
import math
import os
from collections import defaultdict

import numpy as np
from matplotlib import pyplot as plt

# Index of values in the metrics result line
METRIC_IND, MALLOC_IND, THREADS_IND, AVG_THROUGHPUT_IND = 0, 1, 2, -1


def measurement_transform_single(x):
    return round(x / 1e3)


def measurement_transform(m):
    m = list(map(float, m))
    return list(map(measurement_transform_single, m))


def plot_line(x_keys, data, x_label, y_label, title, y_lim_min, y_lim_max, output_file_name=None):
    for attribute, vals in data.items():
        plt.plot(x_keys, measurement_transform(vals), label=attribute, marker='x')

    # Add some text for labels, title and custom x-axis tick labels, etc.
    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.title(title)
    plt.legend(loc='best')  # , ncols=3)
    plt.ylim(y_lim_min, y_lim_max)

    # plt.show()
    plt.savefig(output_file_name or 'image.png')


def get_value(inp):
    return inp.split(':')[1].strip()


def get_output_file_path(input_file_path, result_dir, extension='csv'):
    input_file_name = input_file_path.split('/')[-1]
    res_file_path = os.path.join(result_dir, f"{input_file_name.split('.')[0]}_res.{extension}")

    return res_file_path


def compute_avg_per_thread_perf(input_file_path, data_structure, result_dir):
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
    plot_data = defaultdict(list)
    min_v, max_v = math.inf, -math.inf

    # for threads, mallocs_map in thread_metric_map.items():
    for threads in sorted(thread_metric_map.keys()):
        mallocs_map = thread_metric_map[threads]
        print(f"{threads} threads samples count:", len(mallocs_map[list(mallocs_map.keys())[0]]))
        row = [threads]

        for name in mallocs:
            avg = round(sum(mallocs_map[name]) / len(mallocs_map[name]))
            row.append(avg)
            plot_data[name].append(avg)
            min_v = min(min_v, avg)
            max_v = max(avg, max_v)

        res.append(row)

    os.makedirs(result_dir, exist_ok=True)
    out_file_path = get_output_file_path(input_file_path, result_dir)

    # Write as CSV
    with open(out_file_path, 'w') as handle:
        for row in res:
            handle.write(','.join(map(str, row)) + '\n')

    plot_input = {
        'x_keys': ('1', '4', '8', '16', '32', '64'),
        'data': plot_data,
        'x_label': 'Number of threads',
        'y_label': 'puts/sec in thousands',
        'title': f'{data_structure} puts performance: Average throughput per thread',
        'y_lim_min': max(0, measurement_transform_single(min_v) - 500),
        'y_lim_max': measurement_transform_single(max_v) + 500,
        'output_file_name': get_output_file_path(input_file_path, result_dir, 'png')
    }

    plot_line(**plot_input)


def parse_args():
    parser = argparse.ArgumentParser()

    parser.add_argument(
        dest="input_file_path",
        action="store",
    )

    parser.add_argument(
        dest="data_structure",
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
    compute_avg_per_thread_perf(args.input_file_path, args.data_structure, args.result_dir)
