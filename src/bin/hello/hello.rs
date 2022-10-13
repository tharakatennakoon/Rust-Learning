pub fn sinhala() {
    println!("ආයුබෝවන්");
}

pub fn japanese() {
    println!("こんにちは")
}

// *********************************************************
// since this function is getting called by inner module
// no need to make this public
// *********************************************************
fn english() { 
    println!("hello");
}

fn test() {
    // cannot acccess since not public
    // casual::test();
}

pub mod casual {    
    pub fn english() {
        println!("Hey");
    }

    pub fn greeting() {
        super::sinhala();//relative paths
        super::japanese();//relative path
        crate::hello::english(); // absolute path
    }

    // this function is cannot accessed by supper module
    fn test() {
        print!("test");
    }
}