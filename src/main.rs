fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}
fn multiply(x: i32, y: i32) -> i32 {
    return x * y;
}
fn divide(x: i32, y: i32) -> i32 {
    if y == 0 {
        println!("UNDEFINED");
    } else if x % y != 0 {
        return x % y;
    } else {
        Some(x / y);
    };
    return x / y;
}
fn modulus(x: u32, y: u32) -> u32 {
    return x % y;
}

fn main() {
    loop {
        println!("Please select an operation:");
        println!("0. Exit");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number");

        if choice == 1 {
            println!("Enter first number:");
            let mut num1 = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to read line");
            let num1: i32 = num1.trim().parse().expect("Please type a number");
            println!("Enter second number:");
            let mut num2 = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2: i32 = num2.trim().parse().expect("Please type a number");
            let result = add(num1, num2);
            println!("{} + {} = {}", num1, num2, result);
        } else if choice == 2 {
            println!("Enter first number:");
            let mut num1 = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to real line");
            let num1: i32 = num1.trim().parse().expect("Please type a number");
            println!("Enter second operand:");
            let mut num2 = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2: i32 = num2.trim().parse().expect("Please type a number");
            let result = subtract(num1, num2);
            println!("{} - {} = {}", num1, num2, result);
        } else if choice == 3 {
            println!("Enter first number:");
            let mut num1 = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to read line");
            let num1: i32 = num1.trim().parse().expect("PLease type a number");
            println!("Enter second number:");
            let mut num2 = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2: i32 = num2.trim().parse().expect("Please type a number");
            let result = multiply(num1, num2);
            println!("{} * {} = {}", num1, num2, result);
        } else if choice == 4 {
            println!("Enter first number:");
            let mut num1 = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to read line");
            let num1: i32 = num1.trim().parse().expect("Please type a number");
            println!("Enter second number:");
            let mut num2 = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2: i32 = num2.trim().parse().expect("Please type a number");
            if num2 == 0 {
                println!("Undefined");
            } else {
                let result = divide(num1, num2);
                if num1 % num2 != 0 {
                    println!(
                        "{} / {} = {}, Remainder = {}",
                        num1,
                        num2,
                        result,
                        modulus(
                            (num1 % num2).try_into().unwrap(),
                            (num2 % num1).try_into().unwrap()
                        )
                    );
                } else {
                    println!("{} /{} = {}", num1, num2, result);
                }
                if choice == 0 {
                    println!("Exiting...");
                    break;
                }
            }
        }
    }
}
