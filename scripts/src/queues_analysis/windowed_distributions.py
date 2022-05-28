import os.path as path
import sys

import matplotlib.pyplot

from queues_analysis.pathing import find_target


def main() -> int:
    target = find_target()
    file = path.join(target, "windowed_output.txt")

    t = []
    total_waits = 0
    wait_counts = []
    total_averaged_waits = 0
    average_wait_counts = []

    with open(file) as lines:
        line = lines.readline().strip()
        expected_columns = "# window_left window_right waits averaged_waits averaged_wait_totals"
        assert line == expected_columns, "File failed sanity check. Did the columns change?\nExpected: " + expected_columns + "\nReceived: " + line

        for line in lines:
            tokens = line.split()

            new_time = float(tokens[0])
            new_wait = int(tokens[2])
            new_average_wait = int(tokens[4])

            total_waits += new_wait
            total_averaged_waits += new_average_wait

            t.append(new_time)
            wait_counts.append(new_wait)
            average_wait_counts.append(new_average_wait)

    # TODO: fix the analysis in this respect: the last window is not aggregated.
    # assert total_waits == total_averaged_waits, "Expected the counted averages to equal the total number of events."

    wait_proportions = [wait_count / total_waits for wait_count in wait_counts]
    averaged_wait_proportions = [average_wait_count / total_averaged_waits for average_wait_count in average_wait_counts]

    fig, ax = matplotlib.pyplot.subplots()

    ax.plot(t, wait_proportions, label="Wait distribution")
    ax.plot(t, averaged_wait_proportions, label="Averaged")

    ax.set_xlabel("time ($s$)")
    ax.set_ylabel("proportion")
    ax.legend()

    fig.show()

    percentiles = [0.9, 0.95, 0.99, 0.995]
    wait_count_percentiles = [percentile * total_waits for percentile in percentiles]
    avg_wait_count_percentiles = [percentile * total_averaged_waits for percentile in percentiles]
    wait_percentiles = [0. for _ in percentiles]
    avg_wait_percentiles = [0. for _ in percentiles]
    n_waits = 0
    n_avgs = 0

    for i in range(0, len(wait_counts)):
        for j in range(0, len(percentiles)):
            if n_waits <= wait_count_percentiles[j] < n_waits + wait_counts[i]:
                wait_percentiles[j] = t[i]
            if n_avgs <= avg_wait_count_percentiles[j] < n_avgs + average_wait_counts[i]:
                avg_wait_percentiles[j] = t[i]
        n_waits += wait_counts[i]
        n_avgs += average_wait_counts[i]

    print("{} {} {}".format("Percentile", "wait", "average"))
    for j in range(0, len(percentiles)):
        print("{} {} {}".format(percentiles[j], wait_percentiles[j], avg_wait_percentiles[j]))

    return 0


if __name__ == '__main__':
    sys.exit(main())
