fn main() {
    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");

    let y = plus_one(5);

    println!("The value of y is: {y}");

    println!("The temperature is {}", temperature_converter(56));

    println!("The nth fibonacci number is: {}", fibonacci(10));

    let s = String::from("Hello, world!");

    println!("The first word is: {}", first_word(&s));

    let a: [i32; 7] = [1,2,3,4,5,6,7];
    let b = array_slice(&a);

    println!("The array slice is: {:?} ", b);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
     x + 1
}

fn temperature_converter(x: i32) -> i32 {
    // the formula says c = (F - 32) * 5/9

    let result = (x - 32) * 5/9;
    return result;
}

fn fibonacci(n: u32) -> u32 {
    
    let mut first = 0;
    let mut second = 1;
    let mut next = 0;

    for _ in 1..n {
        next = first + second;
        first = second;
        second = next;    
           
    }

    // let mut count = 1;
    // let mut first = 0;
    // let mut second = 1;
    // let mut next = 0;

    // while count <= n {

    //     if count >= 2 {
    //         next = first + second;
    //         first = second;
    //         second = next;
    //     }
        


    //     count += 1
    // }
    
    return next;

    // if n == 0 {
    //     return 0;
    // }
    // if n == 1 {
    //     return 1;
    // }

    // return fibonacci(n - 1) + fibonacci(n - 2);
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}


fn array_slice(a:&[i32]) -> &[i32] {

    return &a[0..3]
}