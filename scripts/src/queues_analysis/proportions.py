import json
import os.path as path
import sys
from typing import Dict

from queues_analysis.pathing import find_target


def main() -> int:
    target = find_target()
    file = path.join(target, "counts.txt")
    params = {}
    time_in_n: Dict[int, float] = {0: 0}
    time = 0.
    with open(file) as lines:
        params.update(json.loads(lines.readline()[1:]))
        line = lines.readline().strip()
        expected_columns = "# time(s) arrivals departures in_system"
        assert line == expected_columns, "File failed sanity check. Did the columns change?\nExpected: " +expected_columns +"\nReceived: " + line
        last_n = 0
        for line in lines:
            tokens = line.split()

            n = int(tokens[-1])
            new_time = float(tokens[0])
            dt = new_time - time
            time = new_time

            if last_n not in time_in_n:
                time_in_n[last_n] = 0.
            time_in_n[last_n] += dt
            last_n = n

    rho = params["lambda"] / params["mu"]
    for i in range(0, len(time_in_n)):
        expected = (1. - rho) * rho ** i
        print("{} {} {}".format(i, time_in_n[i] * 1. / time, expected))
    return 0


if __name__ == '__main__':
    sys.exit(main())