#[macro_export]
macro_rules! main {
    ($str: tt) => {
        fn main() {
            let now = std::time::Instant::now();
            let test_data = parse(include_str!(concat!(
                "../../data/input",
                stringify!($str),
                ".test.txt"
            )));
            let final_data = parse(include_str!(concat!(
                "../../data/input",
                stringify!($str),
                ".txt"
            )));
            let p1 = (solve_a(&test_data), solve_a(&final_data));
            let p2 = (solve_b(&test_data), solve_b(&final_data));
            let time = now.elapsed().as_millis();
            println!("Day {}", stringify!($str));
            println!("Part one: {}, {}", p1.0, p1.1);
            println!("Part two: {}, {}", p2.0, p2.1);
            println!("Time: {}ms", time);
        }
    };
}
