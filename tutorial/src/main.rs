fn main() {
    println!("Hello, world!");
    //test();
    //add_numbers(20, 30);
    /*
    let number = {
        let x = 3;
        x + 1
    };

    println!("{}", number);
    */

    let result = add_numbers(2,3);
    println!("{}", result)
}

fn add_numbers(x: i32, y: i32) -> i32{
    return x + y;
    //println!("The sum is: {}", x+y);
}

fn test(){
    println!("Test function");
}