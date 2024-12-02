from typing import List, Tuple


def get_lists() -> Tuple[List[int]]:
    '''
    Opens the 'input' file and parses it's contents
    to a list of integers (whole numbers).

    Assumes you are running the script from `Duncan/day_1/python` folder.
    '''
    left = []
    right = []

    with open('../input', 'r') as file:
        lines = file.readlines()

        for line in lines:
            line = line.strip()
            split = line.split()

            left_val = int(split[0])
            right_val = int(split[1])

            left.append(left_val)
            right.append(right_val)

    left.sort()
    right.sort()

    return (left, right)


def part_one(left: List[int], right: List[int]):
    result = 0

    for i in range(0, len(left)):
        left_val = left[i]
        right_val = right[i]

        result += abs(left_val - right_val)

    print(f"Part one result: {result}")


def get_right_count(left_val: int, right: List[int]):
    count = 0
    found_right_val = False

    for right_val in right:
        if right_val == left_val:
            count += 1
            if not found_right_val:
                found_right_val = True
        else:
            if found_right_val:
                break

    return count


def part_two(left: List[int], right: List[int]):
    result = 0

    for left_val in left:
        result += get_right_count(left_val, right) * left_val

    print(f"Part two result: {result}")


def main():
    left, right = get_lists()
    part_one(left, right)
    part_two(left, right)


if __name__ == '__main__':
    main()
