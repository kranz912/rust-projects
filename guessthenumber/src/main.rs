//use               -- import function
//std               -- standard library
//io                -- input/output
use  std::io;
//fn                -- function declaration
//main              -- function name * main is the entry point of your program
fn main() {
    //printlln! -- macro for printing to screen
    println!("Guess the number");
    println!("Please input your guess");
    //let           -- variable declaration
    //mut           -- mutability indicates that a variable is mutable
    //String::new() -- string declaration
    let mut guess = String::new();
    //io::stdin     -- returns a instance of Stdin  a standard handler for the terminal
    //read_line()   -- standard input handler for user input
    //&mut guess    -- pass input value as a reference to guess variable
    //expect()      -- error handler
    io::stdin().read_line(&mut guess).expect("failed to read line");
    println!("You guess: {}", guess);
}
