mod progress;

use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

use progress::Progress;
use progress::ProgressIteratorExt;

fn expensive_calculation<T>(_n: &T) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let mut hash = HashSet::new();

    hash.insert("nimala");
    hash.insert("tharaka");
    hash.insert("senanayake");
    hash.insert("tennakoon");

    println!("Progress ");
    progress::progress(vec.iter(), expensive_calculation);
    println!("\n\n");

    println!("Progress Ex");
    progress::progress_ex(hash.iter(), expensive_calculation);
    println!("\n\n");

    println!("Progress new");
    for n in Progress::new(vec.iter()) {
        expensive_calculation(n);
    }
    println!("\n\n");

    println!("Progress attach to iterator");
    for n in vec.iter().progress() {
        expensive_calculation(n);
    }
    println!("\n\n");

    println!("Progress with bound");
    for n in vec.iter().progress().with_bound() {
        expensive_calculation(n);
    }
    println!("\n\n");
}
