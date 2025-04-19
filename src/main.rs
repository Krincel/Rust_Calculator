fn add(x: f32, y: f32) -> f32 {
    return x + y;
}
fn subtract(x: f32, y: f32) -> f32 {
    return x - y;
}
fn multiply(x: f32, y: f32) -> f32 {
    return x * y;
}
fn divide(x: f32, y: f32) -> f32 {
    if y == 0.0 {
        println!("UNDEFINED");
        return 0.0;
    } else {
        return x / y;
    }
}

fn modulus(x: f32, y: f32) -> f32 {
    return x % y;
}
fn exponent(x: f32, y: f32) -> f32 {
    return x.powf(y.try_into().unwrap());
}

fn main() {
    loop {
        println!("Please select an operation:");
        println!("0. Exit");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Modulus");
        println!("6. Exponent");
        let mut choice: String = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number");

        if choice == 1 {
            println!("Enter first number:");
            let mut num1: String = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to read line");
            let num1 = num1.trim().parse().expect("Please type a number");
            println!("Enter second number:");
            let mut num2: String = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2 = num2.trim().parse().expect("Please type a number");
            let result = add(num1, num2);
            println!("{} + {} = {}", num1, num2, result);
        } else if choice == 2 {
            println!("Enter first number:");
            let mut num1: String = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to real line");
            let num1 = num1.trim().parse().expect("Please type a number");
            println!("Enter second operand:");
            let mut num2: String = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2 = num2.trim().parse().expect("Please type a number");
            let result = subtract(num1, num2);
            println!("{} - {} = {}", num1, num2, result);
        } else if choice == 3 {
            println!("Enter first number:");
            let mut num1: String = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to read line");
            let num1: f32 = num1.trim().parse().expect("PLease type a number");
            println!("Enter second number:");
            let mut num2: String = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2 = num2.trim().parse().expect("Please type a number");
            let result = multiply(num1, num2);
            println!("{} * {} = {}", num1, num2, result);
        } else if choice == 4 {
            println!("Enter first number:");
            let mut num1: String = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to read line");
            let num1 = num1.trim().parse().expect("Please type a number");
            println!("Enter second number:");
            let mut num2: String = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2 = num2.trim().parse().expect("Please type a number");
            if num2 == 0.0 {
                println!("UNDEFINED");
            } else {
                let result = divide(num1, num2);
                if num1 % num2 != 0.0 {
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
                    println!("{} / {} = {}", num1, num2, result);
                }
            }
        } else if choice == 5 {
            println!("Enter first number:");
            let mut num1: String = String::new();
            std::io::stdin()
                .read_line(&mut num1)
                .expect("Failed to read line");
            let num1 = num1.trim().parse().expect("Please type a number");
            println!("Enter second number:");
            let mut num2: String = String::new();
            std::io::stdin()
                .read_line(&mut num2)
                .expect("Failed to read line");
            let num2 = num2.trim().parse().expect("Please type a number");
            let result = modulus(num1, num2);
            println!("{} mod {} = {}", num1, num2, result);
        } else if choice == 6 {
            println!("Enter a base value.");
            let mut base: String = String::new();
            std::io::stdin()
                .read_line(&mut base)
                .expect("Failed to read line");
            let base = base.trim().parse().expect("Please type a number.");
            println!("Enter an index value:");
            let mut index: String = String::new();
            std::io::stdin()
                .read_line(&mut index)
                .expect("Failed to read line");
            let index = index.trim().parse().expect("Please type a number.");
            let result = exponent(base, index);
            println!("{}^{} = {}", base, index, result);
        } else if choice == 0 {
            println!("Exiting...");
            break;
        } else {
            println!("Invalid choice! Enter a number between 0 & 5.");
        }
    }
}
