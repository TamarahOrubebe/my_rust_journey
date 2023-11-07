fn main() {
     let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let tup = (5, 6, 7);
    let (i, j, k) = tup;
    println!("The values i, j, and k are: {i}, {j}, and {k}");
    println!("new one is: {}", 2 * 3);

    let new_i = tup.0;
    let new_j = tup.1;
    let new_k = tup.2;
    println!("{new_i}, {new_j}, {new_k}");

    let a: [u32; 5] = [1, 2, 3, 4, 5];

    println!("{}", a[0]);
    println!("{}", a[1]);
    println!("{}", a[2]);
    println!("{}", a[3]);
    println!("{}", a[4]);
    

    // let b = [3; 5];

    // println!("{}", b);
}
