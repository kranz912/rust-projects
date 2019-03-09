//use               --  import function
//std               --  standard library
//io                --  input/output
//rand::Rng         --  random crate
use  std::io;
use  rand::Rng;
//cmp               --  c++ module for comparing values
//Ordering          --  enum result of a comparison of two numbers
use  std::cmp::Ordering;
//fn                --  function declaration
//main              --  function name * main is the entry point of your program
fn main() {
    //println! -- macro for printing to screen
    println!("Guess the number");
    //rand::thread_rng().gen_range()(<low>, <high>) creates a number between low and high
    // ex: gen_range(1,101) will randomly generate numbers from 1-100
    let _secret_number = rand::thread_rng().gen_range(1,101);
    loop{
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
        //trim          -- removes spaces at the str
        //parse         -- converts str guess to unassigned int 32
        let guess: u32 = match guess.trim().parse(){
          Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);
        //match                 --  compare two variables
        //Ordering::Less        --  checks if A is less than B
        //Ordering::Greater     --  checks if A is greater than B
        //Ordering::Equal       --  checks if A is equal to B
        match guess.cmp(&_secret_number){
            Ordering::Less =>println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal =>{
                println!("You Win");
                break;
            }
        }
    }
}
