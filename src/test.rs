

#[cfg(test)]
mod tests {
    // use crate::{is_valid, parse_amount, char_at};

    // use crate::{turn, step};
    //
    // #[test]
    // fn test_turn() {
    //
    //     // valid
    //     assert_eq!(turn('N', -90), 'W');
    //     assert_eq!(turn('N', -180), 'S');
    //     assert_eq!(turn('N', -270), 'E');
    //     assert_eq!(turn('N', -360), 'N');
    //
    //
    //     assert_eq!(turn('N', 90), 'E');
    //     assert_eq!(turn('N', 180), 'S');
    //     assert_eq!(turn('N', 270), 'W');
    //     assert_eq!(turn('N', 360), 'N');
    // }

    // #[test]
    // fn test_step() {
    //
    //     assert_step('E', (0,0), ('N', 10), 'E', (-10, 0));
    //     assert_step('E', (0,0), ('L', 90), 'N', (0, 0));
    //     assert_step('E', (0,0), ('R', 90), 'S', (0, 0));
    //
    //     assert_step('S', (0,0), ('L', 90), 'E', (0, 0));
    //     assert_step('S', (0,0), ('R', 90), 'W', (0, 0));
    //
    //     assert_step('S', (0,0), ('L', 270), 'W', (0, 0));
    //     assert_step('S', (0,0), ('R', 270), 'E', (0, 0));
    //
    //     // assert_step('E', (0,0), ('N', 10), 'E', (-10, 0))
    //     // assert_step('E', (0,0), ('N', 10), 'E', (-10, 0))
    // }

    // fn assert_step(facing: char, pos: (i32,i32), command: (char, i32), result_facing: char, result_pos: (i32,i32)) {
    //     let mut f = facing;
    //     let mut p = pos;
    //
    //     f = step(command, &mut p, f);
    //
    //     assert_eq!(result_facing, f);
    //     assert_eq!(result_pos.0, p.0);
    //     assert_eq!(result_pos.1, p.1);
    // }

    //
    //
    // #[test]
    // fn test_is_valid_true() {
    //     assert!(is_valid(String::from("1-3 a: abcde")));
    // }
    //
    // #[test]
    // fn test_is_valid_false() {
    //     assert!(!is_valid(String::from("1-3 b: cdefg")));
    //     assert!(!is_valid(String::from("2-9 c: ccccccccc")));
    // }
    //
    // #[test]
    // fn test_read_file() {
    //     let data = read_file_as_byte_data("input.txt");
    //     let stride = data.iter().position(|&it| it as char == '\n').unwrap() as i32;
    //     // assert_eq!(data.len() % 11, 11);
    //
    //     assert_eq!(char_at(&data, 29,0, stride), '.');
    //     assert_eq!(char_at(&data, 30,0, stride), '#');
    //     assert_eq!(char_at(&data, 31,0, stride), '.');
    //     // assert_eq!(char_at(&data, 7,0, stride), '#');
    //     // assert_eq!(char_at(&data, 0,1,stride), '.');
    //     // assert_eq!(char_at(&data, 7+stride,0, stride), '.');
    //
    //     for c in data {
    //         print!("{}", c) // as char)
    //     }
    // }

}