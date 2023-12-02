use std::io;

fn main() {
    println!("Welcome to RusticTrader!");

    loop {
        let mut input = String::new();

        // Receive messages
        println!("Enter a message:");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Identify if it's an order
        if is_order(&input) {
            // Submit the order to the platform
            submit_order(&input);
            println!("Order submitted!");
        } else {
            println!("Not an order. Ignoring.");
        }
    }
}

fn is_order(message: &str) -> bool {
    // Add your logic to identify if the message is an order
    // For simplicity, let's assume any message containing "order" is an order
    message.to_lowercase().contains("order")
}

fn submit_order(order: &str) {
    // Add your logic to submit the order to the trading platform
    println!("Order submitted: {}", order);
}