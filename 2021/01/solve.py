import csv


def get_depths():
    with open("input.txt") as input_file:
        return list(map(lambda row: int(row[0]), csv.reader(input_file)))


def increases_for_window_size(depths, window_size):
    increases = 0

    # since we're checking if `b + c + d > a + b + c`, we can reduce it to `d > a`. we can then simply compare
    # the edges of the window to see if the depths are increasing over time
    left = 0
    right = window_size

    while right < len(depths):
        if depths[right] > depths[left]:
            increases += 1
        left += 1
        right += 1

    return increases


def main():
    depths = get_depths()
    print("Solution for part 1: {}".format(increases_for_window_size(depths, 1)))
    print("Solution for part 2: {}".format(increases_for_window_size(depths, 3)))


if __name__ == "__main__":
    main()
