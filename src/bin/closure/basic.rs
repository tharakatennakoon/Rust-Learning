pub fn basic() {
    let vec_test = vec![1, 2, 3, 4, 5, 6];
    let j = 0;

    // inner function, cannot access local variables
    fn fun(v: &Vec<i32>) {
        println!("{:?}", v);
    }

    fun(&vec_test);

    // Closure, can access local variables
    let closure_annotated = |i: i32| -> i32 {
        println!("{:?}", vec_test);
        i + j
    };

    println!("closure_annotated :{}", closure_annotated(10));

    // Closure, inferred
    let closure_inferred = |i| {
        println!("{:?}", vec_test);
        i + j
    };

    println!("closure_inferred :{}", closure_inferred(20));

    // move vec_test ownership to the closure
    let closure_move = move |i: i32| {
        println!("{:?}", vec_test);
        i + j
    };

    println!("closure_move :{}", closure_move(30));
    // since move is used in closure_move, cannot access vec_test
    //println!("{}", vec_test.len());
}
