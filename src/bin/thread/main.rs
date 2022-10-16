use std::{thread, time::Duration};

fn main(){
    let v_a = [0,10,20,30,40,50,60,70,80,90];
    let v_v:Vec<i32> = vec![0,10,20,30,40,50,60,70,80,90];
    let v_v2:Vec<i32> = vec![0,10,20,30,40,50,60,70,80,90];


    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("From Spowned thread v_a[{i}] : {}, v_v[{i}] : {}", v_a[i], v_v[i]);
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("array count v_a {}", v_a.len());
    // println!("array count v_v {}", v_v.len()); this gives error since we move it to the thread
    println!("array count v_v2 {}", v_v2.len()); // since this is not used in thread, it will not get moved

    for i in 1..5 {
        println!("From Main thread i : {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // check move and drop keywords closure

    handle.join().unwrap();
}