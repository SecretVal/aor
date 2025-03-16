pub trait Day {
    type Res;
    fn parse_input(str: String) -> Self;
    fn solve(&mut self) -> Self::Res;
}
