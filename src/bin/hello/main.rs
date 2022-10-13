mod hello;

fn main(){
    println!("Hello World");

    hello::sinhala(); // relative path
    hello::japanese();

    hello::casual::greeting();
    
    crate::hello::casual::english(); // absolute path
}

// *******************************************************
// this is same as implementing this in the hello.rs file
// use mod keyword to import the module into main.rs
// *******************************************************

/* mod hello {
    pub fn sinhala() {
        println!("ආයුබෝවන්");
    }

    pub fn japanese() {
        println!("こんにちは")
    }

    pub fn english() {
        println!("hello");
    }

    pub mod casual {    å
        pub fn english() {
            println!("Hey");
        }
    }
} */