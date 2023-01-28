pub fn is_visible(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    let my_val = grid[y][x];
    if x == 0 || y == 0 || x == grid[y].len() - 1 || y == grid.len() - 1 {
        return true;
    }
    // from left
    let visible_from_left = {
        let mut visible = true;
        for i in 0..x {
            visible &= grid[y][i] < my_val;
        }
        visible
    };
    let visible_from_right = {
        let mut visible = true;
        for i in (x + 1)..grid[y].len() {
            visible &= grid[y][i] < my_val;
        }
        visible
    };
    let visible_from_top = {
        let mut visible = true;
        for j in 0..y {
            visible &= grid[j][x] < my_val;
        }
        visible
    };
    let visible_from_bottom = {
        let mut visible = true;
        for j in (y + 1)..grid.len() {
            visible &= grid[j][x] < my_val;
        }
        visible
    };
    // println!(
    //     " {},{}, {} {} {} {}",
    //     x, y, visible_from_left, visible_from_right, visible_from_top, visible_from_bottom
    // );

    visible_from_left || visible_from_right || visible_from_top || visible_from_bottom
}

pub fn process_part1(input: &str) -> String {
    let mut grid: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        grid.push(line.as_bytes().iter().map(|b| *b as usize - 48).collect());
    }
    let test = is_visible(&grid, 0, 0);
    let max_height = grid.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = grid
        .iter()
        .enumerate()
        .map(|(j, row)| {
            let max_width = row.len() - 1;
            row.iter()
                .enumerate()
                .map(|(i, _column)| {
                    if i == 0 || i == max_width || j == 0 || j == max_height {
                        true
                    } else {
                        false
                    }
                })
                .collect()
        })
        .collect();
    println!(
        " width {} height {} ",
        visible_trees[0].len(),
        visible_trees.len()
    );

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            visible_trees[y][x] = is_visible(&grid, x, y);
        }
    }

    // dbg!(visible_trees.clone());
    visible_trees
        .iter()
        .flatten()
        .filter(|&&v| v)
        .count()
        .to_string()
}
pub fn process_part2(input: &str) -> String {
    let mut result = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|a, b| b.cmp(a));
    // dbg!(result);
    let sum: u32 = result.iter().take(3).sum();
    sum.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "20111121
12212112
20002121
02221312
10113131
10110201
10102312
10110201";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
