# [(sequenceDepth, rod pattern index)]
RWM_SEQUENCE = [
    (48, 0),
    (48, 1),
    (48, 2),
    (48, 3),
    (48, 4),
    (48, 5),
    (48, 6),
    (4, 7),
    (8, 7),
    (4, 8),
    (12, 7),
    (8, 8),
    (4, 9),
    (16, 7),
    (8, 9),
    (12, 8),
    (20, 7),
    (16, 8),
    (12, 9),
    (20, 8),
    (16, 9),
    (4, 10),
    (4, 11),
    (24, 8),
    (8, 11),
    (24, 7),
    (8, 10),
    (30, 7),
    (12, 11),
    (12, 10),
    (36, 7),
    (16, 11),
    (30, 8),
    (16, 10),
    (20, 11),
    (42, 7),
    (20, 10),
    (36, 8),
    (24, 10),
    (24, 11),
    (48, 7),
    (42, 8),
    (20, 9),
    (48, 8),
    (28, 11),
    (28, 10),
    (24, 9),
    (32, 11),
    (32, 10),
    (28, 9),
    (36, 10),
    (36, 11),
    (40, 10),
    (32, 9),
    (44, 10),
    (40, 11),
    (48, 10),
    (36, 9),
    (44, 11),
    (40, 9),
    (48, 11),
    (44, 9),
    (48, 9),
]
# [[(column, row)]]
ROD_PATTERNS = [
    [
        (18, 23),
        (26, 31),
        (34, 23),
        (26, 15),
        (18, 7),
        (10, 15),
        (2, 23),
        (10, 31),
        (18, 39),
        (34, 39),
        (34, 7),
    ],
    [
        (26, 23),
        (18, 15),
        (10, 23),
        (18, 31),
        (26, 39),
        (34, 31),
        (42, 23),
        (34, 15),
        (26, 7),
        (10, 7),
        (10, 39),
    ],
    [
        (22, 27),
        (30, 19),
        (22, 11),
        (14, 19),
        (6, 27),
        (14, 35),
        (22, 43),
        (30, 35),
        (38, 27),
        (38, 11),
        (6, 11),
    ],
    [
        (22, 19),
        (14, 27),
        (22, 35),
        (30, 27),
        (38, 19),
        (30, 11),
        (22, 3),
        (14, 11),
        (6, 19),
        (6, 35),
        (38, 35),
    ],
    [(42, 27), (26, 3), (2, 19), (18, 43), (42, 19), (18, 3), (2, 27), (26, 43)],
    [(34, 11), (10, 11), (10, 35), (34, 35)],
    [(26, 19), (18, 19), (18, 27), (26, 27)],
    [(14, 7), (6, 31), (30, 39), (38, 15), (6, 15), (14, 39), (38, 31), (30, 7)],
    [(18, 11), (10, 27), (26, 35), (34, 19), (26, 11), (10, 19), (18, 35), (34, 27)],
    [(22, 23), (22, 15), (14, 23), (22, 31), (30, 23)],
    [(22, 7), (6, 23), (22, 39), (38, 23)],
    [(14, 15), (14, 31), (30, 31), (30, 15)],
]

STATE_MAX = -1
for _, rodPatternIndex in RWM_SEQUENCE:
    STATE_MAX += len(ROD_PATTERNS[rodPatternIndex])


# Return (group, rod index) from state.
def decodeState(state):
    group = 1
    length = len(ROD_PATTERNS[RWM_SEQUENCE[group - 1][1]])
    while state >= length:
        state -= length
        group += 1
        length = len(ROD_PATTERNS[RWM_SEQUENCE[group - 1][1]])
    return (group, state)


def rodExists(column, row):
    if not (3 <= row <= 43 and 2 <= column <= 42):
        return False  # Out of range
    if row % 4 != 3 or column % 4 != 2:
        return False  # In-between
    x = (column - 2) // 4
    y = (row - 3) // 4
    if x > 5:
        x = 10 - x  # Mirror x
    if y > 5:
        y = 10 - y  # mirror y
    if x == 0 and y < 4:
        return False  # Outside circle
    if y == 0 and x < 4:
        return False  # Outside circle
    if x == 1 and y == 1:
        return False  # Outside circle
    return True  # Inside circle


def rodPatternIndexLookup(column, row):
    position = (column, row)
    for i, rodPattern in enumerate(ROD_PATTERNS):
        if position in rodPattern:
            return i
    raise Exception(f"Unable to find pattern with rod {column}-{row}.")


def calculateDepth(state, column, row):
    currentGroup, currentRodIndex = decodeState(state)
    currentSequenceDepth = RWM_SEQUENCE[currentGroup - 1][0]
    currentRodPatternIndex = RWM_SEQUENCE[currentGroup - 1][1]
    specifiedRodPatternIndex = rodPatternIndexLookup(column, row)
    # If the specified rod is in the rod group we are pulling now...
    if specifiedRodPatternIndex == currentRodPatternIndex:
        # Iterate over the rods in the current pattern up to and including the one we are pulling now...
        for position in ROD_PATTERNS[currentRodPatternIndex][: currentRodIndex + 1]:
            # If this is the specified rod return the end position of this group in the sequence.
            if position == (column, row):
                return currentSequenceDepth
    # Iterate backwards over the RPM sequence groups...
    for sequenceDepth, rodPatternIndex in reversed(RWM_SEQUENCE[: currentGroup - 1]):
        # If the specified rod is in this group return the end position
        # (all rods in this group have been pulled before).
        if specifiedRodPatternIndex == rodPatternIndex:
            return sequenceDepth
    # No rods have been pulled.
    return 0


def getCurrentRod(state):
    group, rodIndex = decodeState(state)
    return ROD_PATTERNS[RWM_SEQUENCE[group - 1][1]][rodIndex]


def drawFCD(state):
    display = "\x1b[2J"
    group, rodIndex = decodeState(state)
    rodPatternIndex = RWM_SEQUENCE[group - 1][1]
    rodPattern = ROD_PATTERNS[rodPatternIndex]

    for row in range(43, 2, -4):
        """
        >PP<
        XX-YY
        """

        for column in range(2, 43, 4):
            if rodExists(column, row):
                if (column, row) in rodPattern:
                    if rodPattern.index((column, row)) < rodIndex:
                        ansiFormat = "\x1b[102m\x1b[30m"
                    elif rodPattern.index((column, row)) > rodIndex:
                        ansiFormat = "\x1b[104m\x1b[30m"
                    else:
                        ansiFormat = "\x1b[103m\x1b[30m"
                else:
                    ansiFormat = ""
                ansiReset = "\x1b[0m" if ansiFormat != "" else ""
                display += f"{ansiFormat} {calculateDepth(state, column, row):02d}  {ansiReset} "
            elif row == 43:
                if column == 2:
                    display += "State Group Idx Pattern "
            else:
                display += "      "
        display += "\n"

        for column in range(2, 43, 4):
            if rodExists(column, row):
                if (column, row) in rodPattern:
                    if rodPattern.index((column, row)) < rodIndex:
                        ansiFormat = "\x1b[102m\x1b[30m"
                    elif rodPattern.index((column, row)) > rodIndex:
                        ansiFormat = "\x1b[104m\x1b[30m"
                    else:
                        ansiFormat = "\x1b[103m\x1b[30m"
                else:
                    ansiFormat = ""
                ansiReset = "\x1b[0m" if ansiFormat != "" else ""
                display += f"{ansiFormat}{column:02d}-{row:02d}{ansiReset} "
            elif row == 43:
                if column == 2:
                    display += f"{state:03d}   {group:02d}    {rodIndex:02d}  {rodPatternIndex:02d}      "
            else:
                display += "      "
        display += "\n"
    return display


state = 0
while True:
    group, rodIndex = decodeState(state)
    print(drawFCD(state))
    command = input("[n]ext (default), [p]revious, or state number: ").lower()
    if command == "" or command.startswith("n"):
        state += 1
    elif command.startswith("p"):
        state -= 1
    else:
        try:
            state = int(command)
        except ValueError:
            pass
    if state < 0:
        state = 0
    elif state > STATE_MAX:
        state = STATE_MAX
