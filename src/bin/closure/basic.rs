pub fn basic() {
    let mut test_vec = vec![1,2,3,4,5,6];
    let j = 10;

    // cannot access function local variables,
    // need to pass it as arguments
    fn func(v : &Vec<i32>) {
        println!("{:?}", v);
    }

    // since reference is passed, the test_vec can be accessed after calling func,
    func(&test_vec);

    // Closure
    let closure = || {

    };

    // Closure
     let mut closure_mut = |i:i32| {

        test_vec.iter_mut().for_each(|f|{
            *f = *f * j;
        });

        test_vec.len()
    };

    let i = closure_mut(10);
    println!("vec size {}, {:?}", i, test_vec);

}