#[path = "../src/day1.rs"]
mod day1;

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn acceptance_puzzle_1() {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        let result = day1::puzzle1(input);

        assert_eq!(result, 142);
    }
    #[test]
    fn acceptance_puzzle_2() {
        let input = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let result = day1::puzzle2(input);

        assert_eq!(result, 281);
    }

    #[test]
    fn it_handles_a_single_line() {
        let input = "1abc2";

        let result = day1::puzzle1(input);

        assert_eq!(result, 12);
    }

    #[test]
    fn parses_string_of_digits_to_digits () {
        let input ="nineasfdfsatwo";

        let result = day1::puzzle2(input);

        assert_eq!(result, 92);
    }

    #[test]
    fn parses_string_of_overlapping_digits_to_first_digit () {
        let input ="xtwone3four";

        let result = day1::puzzle2(input);

        assert_eq!(result, 24);
    }

    #[test]
    fn it_handles_3_or_more_digits_in_line() {
        let input = "a1b2c3d4e5f";

        let result = day1::puzzle1(input);

        assert_eq!(result, 15);
    }

    #[test]
    fn it_handles_a_single_digit_in_line() {
        let input = "treb7uchet";

        let result = day1::puzzle1(input);

        assert_eq!(result, 77);
    }
}
