import json
import sys
import os.path as path

from queues_analysis.pathing import find_target


def main() -> int:
    target = find_target()
    file = path.join(target, "counts.txt")
    params = {}

    last_arrival = 0.
    n_arrivals = 0
    arrival_time_sum = 0.

    time_of_last_service_start = 0.
    n_departures = 0
    service_time_sum = 0.

    with open(file) as lines:
        params.update(json.loads(lines.readline()[1:]))
        line = lines.readline().strip()
        expected_columns = "# time(s) arrivals departures in_system"
        assert line == expected_columns, "File failed sanity check. Did the columns change?\nExpected: " +expected_columns +"\nReceived: " + line
        last_n = 0
        for line in lines:
            tokens = line.split()

            n = int(tokens[-1])
            time = float(tokens[0])

            if n < last_n:
                service_time = time - time_of_last_service_start
                n_departures += 1
                time_of_last_service_start = time
                service_time_sum += service_time
            else:
                arrival_time = time - last_arrival
                n_arrivals += 1
                last_arrival = time
                arrival_time_sum += arrival_time
                if n == 1:
                    time_of_last_service_start = time
            last_n = n

    print("lambda, actual = {}, sampled = {}".format(params["lambda"], n_arrivals / arrival_time_sum))
    print("mu, actual = {}, sampled = {}".format(params["mu"], n_departures / service_time_sum))
    return 0


if __name__ == '__main__':
    sys.exit(main())