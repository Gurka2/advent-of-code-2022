INPUT = "input"
TEST = "test"
STACKS: int = 9


def parse_part1(indata: list[str]):
    is_parsing_stacks: bool = True

    for line in indata:
        if len(line) > 2 and line[1] == '1':
            is_parsing_stacks = False
        if is_parsing_stacks:
            crate_positions = [(i+1)*4 - 3 for i in range(STACKS)]
            length = len(line)
            for crate_pos in crate_positions:
                if crate_pos < length:
                    print(line[crate_pos])


def main():
    file = open(TEST, "r")
    lines = file.readlines()
    parse_part1(lines)


main()
