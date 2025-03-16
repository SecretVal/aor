pub trait Day {
    type Res;
    fn parse_input(str: String) -> Self;
    fn solve_part1(&mut self) -> Self::Res;
    fn solve_part2(&mut self) -> Self::Res;
}
