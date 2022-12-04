#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello")
    };
}

#[macro_export]
macro_rules! x_and_y {
    (x => $e:expr) => {println!("X is {}", $e)};
    (y => $e:expr) => {println!("Y is {}", $e)};
}

#[macro_export]
macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You are in macro build function {}", stringify!($func_name));
        }
    };
}

#[macro_export]
macro_rules! println_ex {
    ($e:expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    };
}

#[macro_export]
macro_rules! compr {
    ($id1:ident | $id2:ident <- [$start:expr; $end:expr], $count:expr) => {
        {
            let mut temp_vec = Vec::new();

            for num in $start..$end + 1 {
                if $count(num) {
                    temp_vec.push(num);
                }
            }

            temp_vec
        }
    };
}

#[macro_export]
macro_rules! new_map {
    ($($key:expr, $value:expr);*) => {
        {
            let mut hmap = HashMap::new();

            $(
                hmap.insert($key, $value);
            )*

            hmap
        }
    };
}

#[macro_export]
macro_rules! calc_variadic {
    (eval $e:expr) => {
        {
            let _val : usize = $e;

            println!("{} = {}", stringify!($e), $e);
        }
    };

    (eval $e:expr, $(eval $es:expr),+) => {
        {
            calc_variadic!{eval $e}
            calc_variadic!{$(eval $es),+}
        }
    }
}

#[macro_export]
macro_rules! calc_repeat {
    ($(eval $e:expr),+) => {
        {
            $(
                let _val : usize = $e;

                println!("{} = {}", stringify!($e), _val);
            )*
        }
    };
}
