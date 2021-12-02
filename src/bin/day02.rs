use itertools::Itertools;

fn parse(data: &str) -> Vec<(u32, &str)> {
    data.split_whitespace()
        .tuples()
        .map(|(inst, x)| (x.parse().unwrap(), inst))
        .collect::<Vec<_>>()
}

fn solve_a(data: &[(u32, String)]) -> u32 {
    let (x, y) = data.into_iter().fold((0, 0), |a, b| match b.1.as_str() {
        "forward" => (a.0 + b.0, a.1),
        "down" => (a.0, a.1 + b.0),
        "up" => (a.0, a.1 - b.0),
        _ => a,
    });
    x * y
}

fn solve_b(data: &[(u32, String)]) -> u32 {
    let (x, y, _) = data.into_iter().fold((0, 0, 0), |a, b| match b.1.as_str() {
        "forward" => (a.0 + b.0, a.1 + (b.0 * a.2), a.2),
        "down" => (a.0, a.1, a.2 + b.0),
        "up" => (a.0, a.1, a.2 - b.0),
        _ => a,
    });
    x * y
}

aoc2021::main!(2);
