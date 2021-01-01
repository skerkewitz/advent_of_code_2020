use crate::utils;

fn result_part1(data: &Vec<usize>) -> usize {
    for i in &data[0..data.len() - 2] {
        for j in &data[1..data.len() - 1] {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    unreachable!()
}

fn result_part2(data: &Vec<usize>) -> usize {
    for i in &data[0..data.len() - 3] {
        for j in &data[1..data.len() - 2] {
            for k in &data[2..data.len() - 1] {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    unreachable!()
}

pub fn main() -> std::io::Result<()> {
    let data = utils::read_lines_from_file("inputs/day01/input.txt")?.iter()
        .map(utils::string_to_number)
        .collect::<Vec<usize>>();

    utils::assert_and_print_result("day01, part1", 691771, result_part1(&data));
    utils::assert_and_print_result("day01, part2", 232508760, result_part2(&data));
    Ok(())
}


