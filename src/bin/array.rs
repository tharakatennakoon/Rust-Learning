fn main() {
    array();
}

fn array()
{
    let array = [1.0, 2.0, 3.0];
    println!("array[0] is {}", array[0]);

    let d2_array = 
        [[1,2,3],
         [4,5,6]];

    for (_index, items) in d2_array.iter().enumerate() 
    {
        print!("{{");
        for (_index2, item) in items.iter().enumerate()
        {
            print!("{},", item);
        }
        println!("}}");
    }

    let mul_array = [[[10;100]; 20]; 5];

    println!("mul_array[0][0][0] is {}", mul_array[0][0][0]);
}