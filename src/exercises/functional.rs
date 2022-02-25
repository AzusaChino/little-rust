// parent mod
use super::runner;

// count a and b (common function)
pub fn count(a: i32, b: i32) -> i32 {
    a + b
}

fn run_runner() {
    runner::echo("1");
}