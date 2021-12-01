import csv


def get_depths():
    with open("input.txt") as input_file:
        return list(map(lambda row: int(row[0]), csv.reader(input_file)))


def increases_for_window_size(depths, window_size):
    buffer = [None] * window_size  # using as circular buffer
    curr_sum = 0
    increases = 0

    for i in range(len(depths)):
        depth = depths[i]

        buffer_idx = i % window_size
        old_depth = buffer[buffer_idx]
        buffer[buffer_idx] = depth

        new_sum = curr_sum + depth

        if old_depth is not None:
            # we've filled our buffer, therefore we'll need to remove the old value from our running sum
            new_sum -= old_depth

            # since we've seen enough depths, we can start incrementing the increased depths counter as appropriate
            if new_sum > curr_sum:
                increases += 1

        curr_sum = new_sum

    return increases


def main():
    depths = get_depths()
    print("Solution for part 1: {}".format(increases_for_window_size(depths, 1)))
    print("Solution for part 2: {}".format(increases_for_window_size(depths, 3)))


if __name__ == "__main__":
    main()
