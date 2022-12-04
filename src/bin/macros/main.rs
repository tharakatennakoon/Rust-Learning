mod macros;

fn even(i:i32) -> bool {
    i % 2 == 0
}

use std::collections::HashMap;

fn main() {
    hello!();

    x_and_y!(x => 10);
    x_and_y!(y => 30);

    build_fn!(test_macro_build);
    test_macro_build();

    println_ex!(10 + 30 * 5);

    let evens = compr!(x | x <- [0; 10], even);
    println!("evens : {:?}", evens);

    let odds = compr!(x | y <- [0; 10], |i:i32| -> bool {i % 2 != 0});
    println!("odds : {:?}", odds);

    let map = new_map!(
        1,2;
        2,3;
        4,5);
    println!("map {:?}", map);

    println!("Calc Variadic");
    calc_variadic!(
        eval 4 + 5,
        eval 5 * 6,
        eval 10 / 2
    );

    println!("Calc pattern repeat");
    calc_repeat!(
        eval 4 + 5,
        eval 5 * 6,
        eval 10 /2
    );

}