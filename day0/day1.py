# O(n) time, no extra space
def solve(data, window=1):
    return sum(curr > prev for prev, curr in zip(data, data[window:]))

def part_a(data):
    return solve(data, 1)

def part_b(data):
    return solve(data, 3)

def main():
    with open("part1_input.txt", "r", encoding="utf-8") as inputFile:
        data = list(map(int, inputFile.readlines()))

    print(part_a(data))
    print(part_b(data))

if __name__ == "__main__":
    main()
