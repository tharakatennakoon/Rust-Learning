const CLEAR : &str = "\x1B[2J\x1B[1;1H";

pub fn progress<T, Iter>(iter : Iter, f : fn(&T)) 
where Iter:Iterator<Item = T>{
    let mut i = 1;
    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(&n);
    }
}

// same as progress, remove T
pub fn progress_ex<Iter>(iter : Iter, f : fn(&Iter::Item)) 
where Iter:Iterator{
    let mut i = 1;
    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(&n);
    }
}

pub struct Progress<Iter> {
    iter:Iter,
    i:usize,
    bound:Option<usize>
}

impl<Iter> Progress<Iter>{
    pub fn new(iter:Iter) -> Self {
        Progress { iter, i: 0, bound: None }
    }
}

impl <Iter> Progress<Iter>
where Iter:ExactSizeIterator{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Iterator for Progress<Iter> 
where Iter: Iterator {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        match self.bound {
            Some(bound) => {
                println!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i));
            },
            None => {
                println!("{}", "*".repeat(self.i));
            }
        }
        self.i += 1;
        self.iter.next()
    }
}

// add to iterrator
pub trait ProgressIteratorExt: Sized{
    fn progress(self) -> Progress<Self>;    
}

impl<Iter> ProgressIteratorExt for Iter
// to block non iterable types
where Iter:Iterator{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

