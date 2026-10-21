fn main() {
    println!("Hello, world!");
    // Prints the Fibonacci series up to the value 89
    get_fibo_up_to(89); 
}

fn get_fibo_up_to(max_value: i32) {
    let mut a = 0;
    let mut b = 1;

    if max_value >= 0 {
        print!("{a} ");
    }
    
    while b <= max_value {
        print!("{b} ");
        let next = a + b;
        a = b;
        b = next;
    }
    println!(); // Adds a final new line
}
