import csv
from functools import reduce


def get_directions():
    with open("input.txt") as input_file:
        return list(
            map(
                lambda row: (
                    int(row[1]) if row[0] == "forward" else 0,
                    int(row[1]) if row[0] == "down" else int(row[1]) * -1 if row[0] == "up" else 0,
                ),
                csv.reader(input_file, delimiter=" "),
            ),
        )


def direction_multiple(directions):
    return reduce(lambda x, y: x * y, map(sum, zip(*directions)))


def direction_multiple_with_aim(directions):
    depth = 0
    forward = 0
    aim = 0

    for (f, d) in directions:
        forward += f
        aim += d
        depth += aim * f

    return depth * forward


def main():
    directions = get_directions()
    print("Solution for part 1: {}".format(direction_multiple(directions)))
    print("Solution for part 2: {}".format(direction_multiple_with_aim(directions)))


if __name__ == "__main__":
    main()
