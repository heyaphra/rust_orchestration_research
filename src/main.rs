mod task;

use task::Task;
use task::Callable;

fn greet<'a>() -> &'a str {
    let greeter = Task::new(&|| -> &str { "Hello, World!" });
    greeter.run()
}

fn two_x_two<'a>() -> &'a usize {
    let adder = Task::new(&|| -> &usize { &(2 + 2) });
    adder.run()
}
fn main() {
    let greeting: &str = greet();
    print!("\n{}", greeting);

    let two_times_two: &usize = two_x_two();
    print!("\n{}\n", two_times_two);
}
