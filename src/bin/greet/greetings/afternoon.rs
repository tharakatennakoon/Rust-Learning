pub fn description() {
    println!("morning greetings");
}

pub mod formal {
    pub fn english() {
        println!("good afternoon");
    }

    pub fn japanese() {
        println!("こんにちは")
    }
}

pub mod casual;