import json
import os.path
import sys
import os.path as path
from typing import Dict


def find_target() -> str:
    cd = path.abspath(".")
    maybe = path.join(cd, "target")
    while not path.exists(maybe):
        cd = path.abspath(path.join(cd, ".."))
        maybe = path.join(cd, "target")
    return maybe


def main() -> int:
    target = find_target()
    file = path.join(target, "counts.txt")
    params = {}
    counts: Dict[int, int] = {0: 0}
    samples = 0
    with open(file) as lines:
        params.update(json.loads(lines.readline()[1:]))
        lines.readline()
        for line in lines:
            n = int(line.split()[-1])
            if n not in counts:
                counts[n] = 0
            counts[n] += 1
            samples += 1

    rho = params["lambda"] / params["mu"]
    for i in range(0, len(counts)):
        expected = (1. - rho) * rho ** i
        print("{} {} {}".format(i, counts[i] * 1. / samples, expected))
    return 0


if __name__ == '__main__':
    sys.exit(main())