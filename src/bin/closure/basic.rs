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

pub fn capturing() {
    let mut test_vec = vec![1, 2, 3, 4, 5, 6];
    let j = 10;

    // Closure, since we are only accessing test_vec, not need add mut or move
    // this is imuttable borrow
    let closure = || -> i32 { test_vec.iter().sum() };
    println!("[capturing] 1 : Vec Sum : {}", closure());

    // Closure, sice we use test_vec mutable borrowing, need to make closure mutable
    let mut closure_mut = |i: i32| {
        test_vec.iter_mut().for_each(|f| {
            *f = *f * (j + i);
        });
        test_vec.len()
    };

    #[allow(unused_variables)]
    let i = closure_mut(10);

    // this line cause following error
    // cannot borrow `test_vec` as immutable because it is also borrowed as mutable
    // E0502 : https://doc.rust-lang.org/error-index.html#E0502
    // println!("vec size {}, {:?}", i, test_vec);

    let i = closure_mut(20);
    println!("[capturing] 2 : vec size {}, {:?}", i, test_vec);

    // ther test_vec can be moved after last call to closure_mut
    // move test_vec to a Box
    let test_vec_box = Box::new(test_vec);

    let closure_box = || {
        println!("[capturing] 3 closure_box : vec {:?}", test_vec_box);
    };

    closure_box();

    // since closure_box only do immutable borrowing, we can print the values here 
    println!("[capturing] 4 closure_box after : vec {:?}", test_vec_box);

    // drop the test_vec_box
    // since the mem is drop inside of the closure, closure will move the captured variable
    let test_vec_box_clone = test_vec_box.clone();
    let closure_box_drop = || {
        println!("[capturing] 5 closure_box_drop : vec {:?}", test_vec_box);
        println!("[capturing] 6 closure_box_drop : vec clone {:?}", test_vec_box_clone);
        drop(test_vec_box);
    };

    closure_box_drop();

    // compile error due to mem drop
    // println!("[capturing] 6 closure_box_drop after : vec {:?}", test_vec_box);
    // since only test_vec_box is move, test_vec_box_clone can use after calling the closure
    println!("[capturing] 6 closure_box_drop after : vec clone {:?}", test_vec_box_clone);
}

pub fn closure_clone() {
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
    println!("[closure_clone] 1 : vec size {:?}", test_vec);

    // Following line cause error
    // closure cannot be invoked more than once because it moves the variable `test_vec` out of its environment
    // E0382 : https://doc.rust-lang.org/error-index.html#E0382
    // let test_vec = closure1(20);

    test_vec = closure(20);
    println!("[closure_clone] 2 : vec size {:?}", test_vec);

    // Todo
    // check custom structure need to implement clone trait if closure is cloned
}
