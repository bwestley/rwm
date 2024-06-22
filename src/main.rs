use std::io::{self, BufRead, Write};

// [(sequenceDepth, rod pattern index)]
const RWM_SEQUENCE: [(u64, usize); 63] = [
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
    (40, 11),
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
];
// [[(column, row)]]
const ROD_PATTERNS: [[(u64, u64); 11]; 12] = [
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
    [
        (42, 27),
        (26, 3),
        (2, 19),
        (18, 43),
        (42, 19),
        (18, 3),
        (2, 27),
        (26, 43),
        (0, 0),
        (0, 0),
        (0, 0),
    ],
    [
        (34, 11),
        (10, 11),
        (10, 35),
        (34, 35),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ],
    [
        (26, 19),
        (18, 19),
        (18, 27),
        (26, 27),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ],
    [
        (14, 7),
        (6, 31),
        (30, 39),
        (38, 15),
        (6, 15),
        (14, 39),
        (38, 31),
        (30, 7),
        (0, 0),
        (0, 0),
        (0, 0),
    ],
    [
        (18, 11),
        (10, 27),
        (26, 35),
        (34, 19),
        (26, 11),
        (10, 19),
        (18, 35),
        (34, 27),
        (0, 0),
        (0, 0),
        (0, 0),
    ],
    [
        (22, 23),
        (22, 15),
        (14, 23),
        (22, 31),
        (30, 23),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ],
    [
        (22, 7),
        (6, 23),
        (22, 39),
        (38, 23),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ],
    [
        (14, 15),
        (14, 31),
        (30, 31),
        (30, 15),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ],
];
const ROD_PATTERN_LENGTHS: [usize; 12] = [11, 11, 11, 11, 8, 4, 4, 8, 8, 5, 4, 4];

// Return (group, rod index) from state.
fn decode_state(state: usize) -> (usize, usize) {
    let mut group: usize = 1;
    let mut length: usize = ROD_PATTERN_LENGTHS[RWM_SEQUENCE[group - 1].1];
    let mut rod_index = state;
    while rod_index >= length {
        rod_index -= length;
        group += 1;
        length = ROD_PATTERN_LENGTHS[RWM_SEQUENCE[group - 1].1];
    }
    return (group, rod_index);
}

fn rod_exists(column: u64, row: u64) -> bool {
    if !(3 <= row && row <= 43 && 2 <= column && column <= 42) {
        return false;
    } // Out of range
    if row % 4 != 3 || column % 4 != 2 {
        return false;
    } //  In-between
    let mut x: u64 = (column - 2) / 4;
    let mut y: u64 = (row - 3) / 4;
    if x > 5 {
        x = 10 - x;
    } // Mirror x
    if y > 5 {
        y = 10 - y;
    } // mirror y
    if x == 0 && y < 4 {
        return false;
    } // Outside circle
    if y == 0 && x < 4 {
        return false;
    } // Outside circle
    if x == 1 && y == 1 {
        return false;
    } // Outside circle
    return true; // Inside circle
}

fn rod_pattern_index_lookup(column: u64, row: u64) -> usize {
    let position: (u64, u64) = (column, row);
    return ROD_PATTERNS
        .iter()
        .position(|rod_pattern| rod_pattern.contains(&position))
        .expect(&format!("Unable to find pattern with rod {column}-{row}."));
}

fn calculate_depth(state: usize, column: u64, row: u64) -> u64 {
    let (current_group, current_rod_index): (usize, usize) = decode_state(state);
    let current_sequence_depth: u64 = RWM_SEQUENCE[current_group - 1].0;
    let current_rod_pattern_index: usize = RWM_SEQUENCE[current_group - 1].1;
    let specified_rod_pattern_index: usize = rod_pattern_index_lookup(column, row);
    // If the specified rod is in the rod group we are pulling now...
    if specified_rod_pattern_index == current_rod_pattern_index {
        // Iterate over the rods in the current pattern up to and including the one we are pulling now...
        for position in &ROD_PATTERNS[current_rod_pattern_index][0..current_rod_index + 1] {
            // If this is the specified rod return the end position of this group in the sequence.
            if (column, row) == *position {
                return current_sequence_depth;
            }
        }
    }
    // Iterate backwards over the RPM sequence groups...
    for (sequence_depth, rod_pattern_index) in RWM_SEQUENCE[0..current_group - 1].iter().rev() {
        // If the specified rod is in this group return the end position
        // (all rods in this group have been pulled before).
        if specified_rod_pattern_index == *rod_pattern_index {
            return *sequence_depth;
        }
    }
    // No rods have been pulled.
    return 0;
}

fn get_current_rod(state: usize) -> (u64, u64) {
    let (group, rod_index) = decode_state(state);
    return ROD_PATTERNS[RWM_SEQUENCE[group - 1].1][rod_index];
}

fn draw_fcd(state: usize) -> String {
    let mut display: String = "\x1b[2J".to_owned();
    let (group, rod_index) = decode_state(state);
    let rod_pattern_index = RWM_SEQUENCE[group - 1].1;
    let rod_pattern = ROD_PATTERNS[rod_pattern_index];

    for row in (3..44).rev().step_by(4) {
        for column in (2..43).step_by(4) {
            if rod_exists(column, row) {
                let mut ansi_reset = "\x1b[0m";
                let ansi_format = match rod_pattern.iter().position(|rod| rod == &(column, row)) {
                    Some(x) if x < rod_index => "\x1b[102m\x1b[30m",
                    Some(x) if x > rod_index => "\x1b[104m\x1b[30m",
                    Some(_) => "\x1b[103m\x1b[30m",
                    None => {
                        ansi_reset = "";
                        ""
                    }
                };
                display += &format!(
                    "{ansi_format} {:02}  {ansi_reset} ",
                    calculate_depth(state, column, row)
                );
            } else if row == 43 {
                if column == 2 {
                    display += "State Group Idx Pattern ";
                }
            } else {
                display += "      ";
            }
        }
        display += "\n";

        for column in (2..43).step_by(4) {
            if rod_exists(column, row) {
                let mut ansi_reset = "\x1b[0m";
                let ansi_format = match rod_pattern.iter().position(|rod| rod == &(column, row)) {
                    Some(x) if x < rod_index => "\x1b[102m\x1b[30m",
                    Some(x) if x > rod_index => "\x1b[104m\x1b[30m",
                    Some(_) => "\x1b[103m\x1b[30m",
                    None => {
                        ansi_reset = "";
                        ""
                    }
                };
                display += &format!("{ansi_format}{column:02}-{row:02}{ansi_reset} ");
            } else if row == 43 {
                if column == 2 {
                    display += &format!(
                        "{state:03}   {group:02}    {rod_index:02}  {rod_pattern_index:02}      "
                    );
                }
            } else {
                display += "      ";
            }
        }
        display += "\n"
    }
    return display;
}

fn main() {
    let mut state_max: usize = 0;
    for (_, rod_pattern_index) in RWM_SEQUENCE {
        state_max += ROD_PATTERN_LENGTHS[rod_pattern_index];
    }

    state_max -= 1;

    let mut state: usize = 0;
    let mut command: String = String::default();
    let stdin = io::stdin();
    loop {
        print!(
            "{}\n[n]ext (default), [p]revious, or state number: ",
            draw_fcd(state)
        );
        let _ = io::stdout().flush();
        command.clear();
        stdin.lock().read_line(&mut command).unwrap();
        command = command.trim_end().to_lowercase();
        if command == "" || command.starts_with("n") {
            state += 1;
        } else if command.starts_with("p") {
            if state > 0 {
                state -= 1;
            }
        } else {
            state = command.parse::<usize>().unwrap_or(state);
        }
        if state > state_max {
            state = state_max;
        }
    }
}
