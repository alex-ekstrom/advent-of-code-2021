use itertools::Itertools;

fn parse(data: &str) -> Vec<u32> {
    return data.lines().map(|i| i.parse::<u32>().unwrap()).collect();
}

fn solve_a(data: &[u32]) -> u32 {
    return data
        .into_iter()
        .fold((u32::MAX, 0), |a, b| {
            if b > &a.0 {
                (*b, a.1 + 1)
            } else {
                (*b, a.1)
            }
        })
        .1;
}

fn solve_b(data: &[u32]) -> u32 {
    return data
        .into_iter()
        .tuple_windows::<(_, _, _)>()
        .fold((u32::MAX, 0), |a, b| {
            let sum = b.0 + b.1 + b.2;
            return if sum > a.0 {
                (sum, a.1 + 1)
            } else {
                (sum, a.1)
            };
        })
        .1;
}

aoc2021::main!(1);
