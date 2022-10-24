// Fn: the closure uses the captured value by reference (&T)
// FnMut: the closure uses the captured value by mutable reference (&mut T)
// FnOnce: the closure uses the captured value by value (T)

// this function accept a closure as a input parameter.
// closure does not take input arguments and does not return
fn run_closure_fnonce<F>(f : F) 
where F : FnOnce() {
    f();
}

fn run_closure_fn<F>(f : F) 
where F : Fn() {
    f();
}

fn run_closure_fnmut<F>(mut f : F) 
where F : FnMut() {
    f();
}

pub fn test_closure() {
    let mut test_vec = vec![1, 2, 3, 4, 5, 6];
    let j = 10;

    // imutable borrowing, since we are not changing or dropping test_vec
    let closure = || {
        println!("closure : {:?}", test_vec);
    };

    // even FnOnce is used, since closure capturing variable as imutable borrowing 
    run_closure_fnonce(closure);
    run_closure_fnonce(closure);
    run_closure_fn(closure);
    run_closure_fn(closure);
    run_closure_fnmut(closure);
    run_closure_fnmut(closure);

    let mut test_vec_clone  = test_vec.clone();
    // mutable borrowing
    let closure_mut = || {
        test_vec.iter_mut().for_each(|f| {
            *f = *f * j;
        });

        println!("closure_mut : {:?}", test_vec);
    };

    let closure_once = || {
        test_vec_clone.iter_mut().for_each(|f| {
            *f = *f * j;
        });

        println!("closure_mut : {:?}", test_vec_clone);

        drop(test_vec_clone);
    };

    // cannot clone &mut Vec
    // if we use move keyword this can be done
    //let mut closure_mut_clone = closure_mut.clone();

    // Since the closure is mutable, need to use FnMut or FnOnce
    //run_closure_fn(closure_mut);

    // run_closure_fnmut cause test_vec to move,
    // therefore can only call once
    run_closure_fnmut(closure_mut);
    //run_closure_fnmut(closure_mut);

    run_closure_fnonce(closure_once);
    //run_closure_fnonce(closure_mut);
}