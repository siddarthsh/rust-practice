pub fn run(){
    // Print to console
    println!("Hello from print.rs file");


    // Basic formatting
    println!("{} is from {}",
        "Siddarth",
        "Chandigarh");


    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}",
        "Siddarth",
        "Chandigarh",
        "code");


    // Named Arguements
    println!("{name} likes to {activity}",
        name = "Siddarth",
        activity = "Code"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",
        10,
        10,
        10);
}