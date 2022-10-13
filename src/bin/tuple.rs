fn tuple()
{
    let tuple = (1,2.0,"tuple");
    println!("tuple values are {}, {}, {}", tuple.0, tuple.1, tuple.2);


    let (a, b, _c) = tuple;
    println!("tuple values are {}, {}", a, b);
}

fn multi_return(num: i32) -> (i32, i32)
{
    // last expression will get returned, otherwise user return keyword
    // do not use ';' at the end, it will change the expression to statment.
    (num, num * num)
}

fn main() {
    // tupels
    tuple();

    // tuple return fn
    let t = multi_return(13);
    println!("Values are {:?}", t);
}
