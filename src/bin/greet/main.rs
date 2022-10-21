mod greetings;

use greetings::{afternoon, morning};

fn main() {
    println!("Hello, world!");

    morning::description();

    morning::formal::english();
    morning::casual::english();

    morning::formal::japanese();
    morning::casual::japanese();

    afternoon::description();

    afternoon::formal::english();
    afternoon::casual::english();

    afternoon::formal::japanese();
    afternoon::casual::japanese();
}