use std::io;
 
fn main() {
    println!("Welcome to my calculator!");
    let mut operation=String::new();
    let mut a = String::new();
    let mut b=String::new();
    let mut ans=0;

    println!("Enter an operation: +, -, *, /");
    io::stdin().read_line(&mut operation).expect("failed to readline");

    if operation.trim() != "+" && operation.trim() != "-" && operation.trim() != "*" && operation.trim() != "/" {
        println!("Invalid operation");
        return;
    }
 
    println!("Enter first number:");
    io::stdin().read_line(&mut a).expect("failed to readline");

    println!("Enter second number:");
    io::stdin().read_line(&mut b).expect("failed to readline");

    if operation.trim() == "+" {
        ans=a.trim().parse::<i32>().unwrap() + b.trim().parse::<i32>().unwrap();
    }
    else if operation.trim() == "-" {
        ans=a.trim().parse::<i32>().unwrap() - b.trim().parse::<i32>().unwrap();
    }
    else if operation.trim() == "*" {
        ans=a.trim().parse::<i32>().unwrap() * b.trim().parse::<i32>().unwrap();
    }
    else if operation.trim() == "/" {
        if b.trim().parse::<i32>().unwrap() == 0 {
            println!("Cannot divide by zero");
            return;
        }
        ans=a.trim().parse::<i32>().unwrap() / b.trim().parse::<i32>().unwrap();
    }

    println!("The answer is {}", ans);
    println!("--------------------------------");
    println!("Do you want to continue? y/n");
    operation.clear();
    io::stdin().read_line(&mut operation).expect("failed to readline");
    
    while operation.trim() != "y" && operation.trim() != "n" {
        println!("Invalid input");
        println!("Do you want to continue? y/n");
        io::stdin().read_line(&mut operation).expect("failed to readline");
    }

    if operation.trim() == "y" {
        main();
    }
    else{
        println!("Goodbye!");
    }
}