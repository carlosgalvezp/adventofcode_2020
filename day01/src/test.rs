mod lib;

#[cfg(test)]
mod tests {
    fn get_test_contents() -> String{
        return concat!(
            "1721\n",
            "979\n",
            "366\n",
            "299\n",
            "675\n",
            "1456\n",
        ).to_owned();
    }

    #[test]
    fn test_part1() {
        assert_eq!(514578, lib::part1(get_test_contents()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(241861950, lib::part2(get_test_contents()));
    }
}
