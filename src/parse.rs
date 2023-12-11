pub fn char_grid_iter<T: TryFrom<char>>(
    input: &str,
) -> impl Iterator<Item = (usize, usize, T)> + '_ {
    input
        .trim()
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| c.try_into().ok().map(|item| (x, y, item)))
        })
}
