struct Mapper {
    pub map: fn(i32) -> i32,
}
impl Mapper {
    fn new(map: fn(i32) -> i32) -> Self {
        Self { map }
    }
}
pub struct Parent {
    operation: Mapper,
    round: i32,
}
pub fn process_part1(input: &str) -> String {
    "ok".to_string()
}
pub fn process_part2(input: &str) -> String {
    "ok".to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15";
    #[test]
    fn test_closure_in_structure() {
        let foo = Mapper { map: |a| a + 1 };
        (foo.map)(42);
        println!(" result {}", (foo.map)(42));

        (Mapper::new(|a| a + 1).map)(42);
        println!(" result {}", (foo.map)(42));
        let mut m = Parent {
            operation: Mapper { map: |a| a * 2 },
            round: 0,
        };
        m.round += 1;
        println!(" operation {}", (m.operation.map)(42));
        assert_eq!((m.operation.map)(42), 84);
        m.operation = Mapper { map: |a| a + 1 };
        assert_eq!((m.operation.map)(42), 43);

        // let result = process_part1(INPUT);
        // assert_eq!(result, "13140");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "13140");
    }
}
