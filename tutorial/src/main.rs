fn main() {
    let mut tup: (i32, bool, char) = (1, true, 's');

    tup.0 = 10;
    tup = (15, false, 'w');

    println!("{}", tup.0);

    let mut arr: [i32; 5] = [1,2,3,4,5];
    arr[0] = 3;
    println!("{}", arr[4]);
}
