mod greetings;

use greetings::{afternoon, morning};

fn main() {
    println!("Hello, world!");

    morning::formal::japanese();
    morning::casual::japanese();

    afternoon::formal::english();
    afternoon::casual::english();
}