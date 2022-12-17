use itertools::Itertools;

const CORNER_CHAR: char = '+';

pub fn count(lines: &[&str]) -> u32 {
    let mut rectangles_count = 0;
    for line in lines.iter().enumerate() {
        let index = line.0;
        let line = line.1;
        if valid_line(line) {
            let corner_pairs = line.match_indices(CORNER_CHAR).combinations(2);
            for corner_pair in corner_pairs {
                dbg!(&corner_pair);
                for &l in &lines[index + 1..] {
                    let index = corner_pair[0].0;
                    let l: Vec<_> = l.chars().collect();
                    let rect_side_one = l[index];
                    let index_two = corner_pair[1].0;
                    let rect_side_two = l[index_two];
                    let in_between_chars = &l[index..=index_two];
                    if not_side(rect_side_one) || not_side(rect_side_two) {
                        break;
                    }
                    if rect_side_one == CORNER_CHAR
                        && rect_side_two == CORNER_CHAR
                        && !in_between_chars.contains(&' ')
                    {
                        println!("Rectangle found!");
                        rectangles_count += 1;
                    }
                }
            }
        }
    }

    rectangles_count
}

fn valid_line(line_input: &str) -> bool {
    line_input.chars().any(|c| matches!(c, '+' | '-' | '|'))
}

fn not_side(x: char) -> bool {
    x == ' ' || x == '-'
}
