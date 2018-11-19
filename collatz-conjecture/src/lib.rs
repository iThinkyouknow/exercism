fn solve(next_n: u64, steps: u64) -> Option<u64> {
    let next_step = steps + 1;
    match next_n {
        0 => None,
        1 => Some(steps),
        x if x % 2 == 0 => solve(x / 2, next_step),
        x => solve(3 * x + 1, next_step),
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    solve(n, 0)
}
