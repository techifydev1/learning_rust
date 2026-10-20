fn main() {
    let mut number = 10;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let numbers = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index != numbers.len() {
        println!("The value is: {}", numbers[index]);

        index+=1;
    }

    for element in numbers {
        println!("The value is: {}", element);
    }

    for nums in 1..4 {
        println!("{nums}!")
    }

    println!("LIFTOFF!!")
}
