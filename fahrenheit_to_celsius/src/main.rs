fn main() {
    let to_celsius = to_celsius(68);
    println!("to_celsius: {to_celsius}");

    let to_fahrenheit = to_fahrenheit(to_celsius);
    println!("to_fahrenheit: {}", to_fahrenheit);
}


fn to_celsius(f: i32) -> i32 {
    return (f - 32) * 5/9;
}

fn to_fahrenheit(c: i32) -> i32 {
    return (-160 - 9 * c)/-5;
}

// ((9 * c) + 160)/5