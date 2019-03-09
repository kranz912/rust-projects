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
       -- Unlike immutable variables, mutable variables allows us to change it's value
       example of a mutable variable:
    */
    let mut _z =10;
    _z= 20;
    println!("z: {}",_z);

    /*
        Constant Variables
        -- Constants are immutable variables
        -- you can't declare a mutable constant
        -- Constants can be declared in the global scope
        -- Data type is required when declaring a constant variable
    */
    const MAX_POINTS: u32 = 300_000;
    println!("MAX_POINTS: {}",MAX_POINTS);
}
