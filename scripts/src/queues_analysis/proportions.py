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
        assert line == expected_columns, "File failed sanity check. Did the columns change?\nExpected: " + expected_columns + "\nReceived: " + line
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
    steady_number_of_customers = 0
    for n in range(0, len(time_in_n)):
        expected_p_n = (1. - rho) * rho ** n
        measured_p_n = time_in_n[n] * 1. / time
        print("{} {} {}".format(n, measured_p_n, expected_p_n))
        steady_number_of_customers += n * measured_p_n

    # For M/M/1 this comes from 3.2.4
    theoretical = params["lambda"] / (params["mu"] - params["lambda"])

    print("")
    print("Number of customers in system at steady state: measured = {}, theoretical = {}".format(steady_number_of_customers, theoretical))

    return 0


if __name__ == '__main__':
    sys.exit(main())
