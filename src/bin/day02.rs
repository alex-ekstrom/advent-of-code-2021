fn parse(data: &str) -> Vec<(u32, String)> {
    return data
        .lines()
        .map(|i| {
            let mut spl = i.split(' ');
            let inst = spl.next().unwrap().to_string();
            return (spl.next().unwrap().parse::<u32>().unwrap(), inst);
        })
        .collect();
}

fn solve_a(data: &[(u32, String)]) -> u32 {
    let dist = data.into_iter().fold((0, 0), |a, b| match b.1.as_str() {
        "forward" => (a.0 + b.0, a.1),
        "down" => (a.0, a.1 + b.0),
        "up" => (a.0, a.1 - b.0),
        _ => a,
    });
    return dist.0 * dist.1;
}

fn solve_b(data: &[(u32, String)]) -> u32 {
    let dist = data.into_iter().fold((0, 0, 0), |a, b| match b.1.as_str() {
        "forward" => (a.0 + b.0, a.1 + (b.0 * a.2), a.2),
        "down" => (a.0, a.1, a.2 + b.0),
        "up" => (a.0, a.1, a.2 - b.0),
        _ => a,
    });
    return dist.0 * dist.1;
}

aoc2021::main!(2);
