pub fn basic() {
    let vec_test = vec![1, 2, 3, 4, 5, 6];
    let j = 0;

    // inner function, cannot access local variables
    // since reference is passed, the test_vec can be accessed after calling func,
    fn fun(v: &Vec<i32>) {
        println!("[basic] 1 : {:?}", v);
    }

    fun(&vec_test);

    // Closure, can access local variables
    let closure_annotated = |mut i: i32| -> i32 {
        println!("[basic] 2 : {:?}", vec_test);
        i = i + j;
        i
    };

    println!("[basic] 3 : closure_annotated :{}", closure_annotated(10));

    // Closure, inferred
    let closure_inferred = |i| {
        println!("[basic] 4 : {:?}", vec_test);
        i + j
    };

    println!("[basic] 5 : closure_inferred :{}", closure_inferred(20));

    // move vec_test ownership to the closure
    let closure_move = move |i| {
        println!("[basic] 6 : {:?}", vec_test);
        i + j
    };

    println!("[basic] 7 : closure_move :{}", closure_move(30));
    // since move is used in closure_move, cannot access vec_test
    //println!("{}", vec_test.len());
}

pub fn basic1() {
    let mut test_vec = vec![1, 2, 3, 4, 5, 6];
    let j = 10;

    // Closure, since we are only accessing test_vec, not need add mut or move
    // this is imuttable borrow
    let closure = || -> i32 { test_vec.iter().sum() };
    println!("[basic1] 1 : Vec Sum : {}", closure());

    // Closure, sice we use test_vec mutable borrowing, need to make closure mutable
    let mut closure_mut = |i: i32| {
        test_vec.iter_mut().for_each(|f| {
            *f = *f * (j + i);
        });
        test_vec.len()
    };

    let i = closure_mut(10);

    // this line cause following error
    // cannot borrow `test_vec` as immutable because it is also borrowed as mutable
    // E0502 : https://doc.rust-lang.org/error-index.html#E0502
    // println!("vec size {}, {:?}", i, test_vec);

    let i = closure_mut(20);
    println!("[basic1] 2 : vec size {}, {:?}", i, test_vec);
}

pub fn basic2() {
    let mut test_vec = vec![1, 2, 3, 4, 5, 6];
    let j = 10;

    // Closure, since we are moving ownership, no need to make closure mutable
    let closure = move |i: i32| -> Vec<i32> {
        test_vec.iter_mut().for_each(|f| {
            *f = *f * (j + i);
        });
        test_vec
    };

    // this will make a copy of closure.
    // i.e will clone test_vec as well
    // no effect on the calling order
    // However calling to any closure will move ownership
    let closure1 = closure.clone();

    test_vec = closure1(10);
    println!("[basic2] 1 : vec size {:?}", test_vec);

    // Following line cause error
    // closure cannot be invoked more than once because it moves the variable `test_vec` out of its environment
    // E0382 : https://doc.rust-lang.org/error-index.html#E0382
    // let test_vec = closure1(20);

    test_vec = closure(20);
    println!("[basic2] 2 : vec size {:?}", test_vec);
}
