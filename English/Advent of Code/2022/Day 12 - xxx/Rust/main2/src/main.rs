fn main() {
    let input = include_str!("../../../input.txt");

    let mut height_map: Vec<Vec<i64>> = vec![];

    let mut location_init: [i64; 2] = [0; 2];
    let mut location_dest: [i64; 2] = [0; 2];
    for (curr_row, line) in input.trim().split('\n').enumerate() {
        let mut current_row: Vec<i64> = vec![];
        for (curr_col, x) in line.chars().enumerate() {
            if x == 'S' {
                location_init = [curr_col as i64, curr_row as i64];
            } else if x == 'E' { 
                location_dest = [curr_col as i64, curr_row as i64];
            }
            current_row.push(char_value(x));
        }
        height_map.push(current_row);
    }

    unsafe { println!("{}", find_path(&height_map, location_init, location_dest).unwrap() - 1) }

}

unsafe fn find_path(height_map: &Vec<Vec<i64>>, location_curr: [i64; 2], location_dest: [i64; 2] ) -> Option<i64> {
    let mut result: Option<i64> = None;
    VISITED.push(location_curr);

    if location_curr == location_dest {
        result = Some(0);
    } else {
        if (location_curr[0] >= 1) && (height_map[location_curr[1] as usize][(location_curr[0] - 1) as usize] - height_map[location_curr[1] as usize][location_curr[0] as usize] <= 1) && !VISITED.contains(&[location_curr[0] - 1, location_curr[1]]) {
            if let Some(t) = find_path(height_map, [location_curr[0] - 1, location_curr[1]], location_dest) {
                if let Some(res) = result {
                    if t < res { result = Some(t); }
                } else { result = Some(t); }
            }
        }
        if (location_curr[0] < (height_map[0].len() as i64 - 1)) && (height_map[location_curr[1] as usize][(location_curr[0] + 1) as usize] - height_map[location_curr[1] as usize][location_curr[0] as usize] <= 1) && !VISITED.contains(&[location_curr[0] + 1, location_curr[1]]) {
            if let Some(t) = find_path(height_map, [location_curr[0] + 1, location_curr[1]], location_dest) {
                if let Some(res) = result {
                    if t < res { result = Some(t); }
                } else { result = Some(t); }
            }
        }
        if (location_curr[1] >= 1) && (height_map[(location_curr[1] - 1) as usize][location_curr[0] as usize] - height_map[location_curr[1] as usize][location_curr[0] as usize] <= 1) && !VISITED.contains(&[location_curr[0], location_curr[1] - 1]) {
            if let Some(t) = find_path(height_map, [location_curr[0], location_curr[1] - 1], location_dest) {
                if let Some(res) = result {
                    if t < res { result = Some(t); }
                } else { result = Some(t); }
            }
        }
        if (location_curr[1] < (height_map.len() as i64 - 1)) && (height_map[(location_curr[1] + 1) as usize][location_curr[0] as usize] - height_map[location_curr[1] as usize][location_curr[0] as usize] <= 1) && !VISITED.contains(&[location_curr[0], location_curr[1] + 1]) {
            if let Some(t) = find_path(height_map, [location_curr[0], location_curr[1] + 1], location_dest) {
                if let Some(res) = result {
                    if t < res {result = Some(t);}
                } else {result = Some(t);}
            }
        }
    }

    if let Some(t) = result {
        result = Some(t + 1);
    }

    VISITED.remove(VISITED.len() - 1);
    result
}

fn char_value(character: char) -> i64 {

    match character {
        'S' => 0_i64,
        'E' => 25_i64,
        _ => {
            let mut curr_val: i64 = 0;
            for x in ALPHABET.chars() {
                if x == character {
                    break;
                }
                curr_val += 1;
            }
            curr_val
        }
    }

}

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
static mut VISITED: Vec<[i64; 2]> = vec![];