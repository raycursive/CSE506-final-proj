import sys
from io import StringIO
import pandas as pd


def parse_output_to_csv(ds, testcase, alloc, r):
    with open(f"out/{ds}-{testcase}-{alloc}.out.{r}") as f:
        lines = f.readlines()
        
    result = ["n_threads,throughput,usertime,systemtime,cpuusage,max_rss,minor_pf,voluntary_ctx_switch"]
    for _chunk in range(0, len(lines), 4):
        chunk = lines[_chunk:_chunk+4]
        n_threads = int(chunk[1].split(',')[1].split(' ')[-1])
        avg_throughput = float(chunk[1].split(',')[-1].split(' ')[-1])
        utime, stime, cpu, max_rss, minor_pf, voluntary_ctx_switch = [float(x) if not x.endswith("%") else float(x[:-1])/100 for x in chunk[2].split(',')]
        result.append(f"{n_threads},{avg_throughput},{utime},{stime},{cpu},{max_rss},{minor_pf},{voluntary_ctx_switch}\n")

    csv_content = StringIO("\n".join(result))
    return pd.read_csv(csv_content)

(ds, testcase, alloc) = sys.argv[1].split('-')
df_avg = sum(parse_output_to_csv(ds, testcase, alloc, r+1) for r in range(5)) / 5
df_avg.to_csv(f"csv/{ds}-{testcase}-{alloc}.csv", index=False)
