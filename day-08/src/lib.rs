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
pub fn view(grid: &Vec<Vec<usize>>, x: usize, y: usize) -> u32 {
    let my_val = grid[y][x];

    // from left
    let visible_to_left = {
        let mut visible = 0;
        for i in (0..x).rev() {
            visible = visible + 1;
            if grid[y][i] >= my_val {
                break;
            }
        }
        visible
    };
    let visible_to_right = {
        let mut visible = 0;
        for i in (x + 1)..grid[y].len() {
            visible = visible + 1;
            if grid[y][i] >= my_val {
                break;
            }
        }
        visible
    };
    let visible_to_top = {
        let mut visible = 0;
        for j in (0..y).rev() {
            visible = visible + 1;
            if grid[j][x] >= my_val {
                break;
            }
        }
        visible
    };
    let visible_to_bottom = {
        let mut visible = 0;
        for j in (y + 1)..grid.len() {
            visible = visible + 1;
            if grid[j][x] >= my_val {
                break;
            }
        }
        visible
    };
    // println!(
    //     "{}, {},{}, {} {} {} {}",
    //     my_val, y, x, visible_to_left, visible_to_right, visible_to_top, visible_to_bottom
    // );
    if x == 0 || y == 0 || x == grid[y].len() - 1 || y == grid.len() - 1 {
        0
    } else {
        visible_to_left * visible_to_right * visible_to_top * visible_to_bottom
    }
}
pub fn process_part1(input: &str) -> String {
    let mut grid: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        grid.push(line.as_bytes().iter().map(|b| *b as usize - 48).collect());
    }

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
    let mut grid: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        grid.push(line.as_bytes().iter().map(|b| *b as usize - 48).collect());
    }
    // let mut max_view = 0;
    // for (y, row) in grid.iter().enumerate() {
    //     for (x, _cell) in row.iter().enumerate() {
    //         let this_view = view(&grid, x, y);
    //         if this_view >= max_view {
    //             max_view = this_view
    //         }
    //     }
    // }

    let vmax = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _val)| view(&grid, x, y))
                .max()
        })
        .max();

    vmax.unwrap().unwrap().to_string()
    //  max_view.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]

    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "8");
    }
}
