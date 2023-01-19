use std::collections::HashSet;
pub fn process_part1(input: &str) -> String {
    let mut curr = 0;
    let total = input
        .split("\n")
        .map(|item| {
            let s1 = &item[0..item.len() / 2];
            let s2 = &item[item.len() / 2..];
            let mut hash_set1: HashSet<u8> = HashSet::with_capacity(s1.len() * 2);
            let mut hash_set2: HashSet<u8> = HashSet::with_capacity(s2.len() * 2);

            for c in s1.as_bytes() {
                hash_set1.insert(*c);
            }
            for c in s2.as_bytes() {
                hash_set2.insert(*c);
            }
            for c in hash_set1.intersection(&hash_set2) {
                curr = *c;
            }
            match curr {
                b'a'..=b'z' => (curr - b'a' + 1) as i32,

                b'A'..=b'Z' => (curr - b'A' + 27) as i32,
                _ => 0 as i32,
            }
        })
        .sum::<i32>();
    println!(" total {}", total);
    total.to_string()
}
pub fn process_part2(input: &str) -> String {
    let mut total = 0;
    let input_strings = input.lines();
    let elves: Vec<&str> = input_strings.collect();
    for i in (0..elves.len()).step_by(3) {
        let mut hash_set1: HashSet<u8> = HashSet::with_capacity(elves[i].len() * 2);
        let mut hash_set2: HashSet<u8> = HashSet::with_capacity(elves[i + 1].len() * 2);
        let mut hash_set3: HashSet<u8> = HashSet::with_capacity(elves[i + 2].len() * 2);
        for c in elves[i].as_bytes() {
            hash_set1.insert(*c);
        }
        for c in elves[i + 1].as_bytes() {
            hash_set2.insert(*c);
        }
        for c in elves[i + 2].as_bytes() {
            hash_set3.insert(*c);
        }
        for c in hash_set1.intersection(&hash_set2) {
            if hash_set3.contains(c) {
                total = total
                    + match *c {
                        b'a'..=b'z' => (*c - b'a' + 1) as i32,

                        b'A'..=b'Z' => (*c - b'A' + 27) as i32,

                        _ => 0 as i32,
                    };
            }
        }
    }
    total.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    #[ignore]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "70");
    }
}
