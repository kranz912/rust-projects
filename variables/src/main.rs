fn main() {
    /*
        Immutable Variables
        --  All variables are immutable by default
        --  when a variable is immutable once a value is assigned to it you can't change the value
        --  you can declare a immutable variable and assign a value on it but can't change the value
        example of a immutable variable:
    */
    let _x = 5;
    let _y;
        _y= 5;
    println!("x: {}", _x);
    println!("y: {}", _y);

    /*
       Mutable Variables
       --   Unlike immutable variables, mutable variables allows us to change it's value
       example of a mutable variable:
    */
    let mut _z =10;
    _z= 20;
    println!("z: {}",_z);

    /*
        Constant Variables
        --  Constants are immutable variables
        --  you can't declare a mutable constant
        --  Constants can be declared in the global scope
        --  Data type is required when declaring a constant variable
    */
    const MAX_POINTS: u32 = 300_000;
    println!("MAX_POINTS: {}",MAX_POINTS);
    /*
        Shadowing
        --  Creating a copy of the same variable with the same name
        --  the difference between mutable and shadowing
        ----    shadowing creates a new instance of that variable into memory (changes the reference of that variable)
        ----    mutable changes the value of a variable while retaining it's reference
        example of shadowing:
    */
    // assign 5 to s
    let s = 5;
    println!("s: {}",s);

    // s is now 15
    let s = s + 10;
    println!("s: {}",s);

    // s is now 30
    let s = s * 2;
    println!("s: {}",s);
}
