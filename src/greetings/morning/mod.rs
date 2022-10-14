pub fn description() {
    println!("morning greetings");
}

pub mod formal {
    pub fn english() {
        println!("good morning");
    }

    pub fn japanese() {
        println!("こんにちは")
    }
}

pub mod casual;