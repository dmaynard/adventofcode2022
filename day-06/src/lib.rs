use std::collections::HashSet;
pub fn process_part1(input: &str) -> usize {
    let mut chars: Vec<u8> = input.to_string().into_bytes();
    let mut hash_set: HashSet<u8> = HashSet::new();
    let mut marker: usize = 0;
    for i in 3..chars.len() {
        let mut num_unique = 0;
        hash_set.drain();
        for j in i - 3..=i {
            if hash_set.insert(chars[j]) {
                num_unique = num_unique + 1;
            }
        }
        if num_unique == 4 {
            marker = i;
            break;
        }
    }
    marker + 1
}
pub fn process_part2(input: &str) -> usize {
    let mut chars: Vec<u8> = input.to_string().into_bytes();
    let mut hash_set: HashSet<u8> = HashSet::new();
    let mut marker: usize = 0;
    for i in 13..chars.len() {
        let mut num_unique = 0;
        hash_set.drain();
        for j in i - 13..=i {
            if hash_set.insert(chars[j]) {
                num_unique = num_unique + 1;
            }
        }
        if num_unique == 14 {
            marker = i;
            break;
        }
    }
    marker + 1
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]

    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, 7);
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]

    fn part2_works() {
        assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
