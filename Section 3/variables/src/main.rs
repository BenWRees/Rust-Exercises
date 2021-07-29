
//CONSTANTS
/*
    Differences between Constants and Variables:
        - cannot use mut trait with constants 
        -declare constants with the const keyword instead of the let keyword
        - constants must be annotated with an explicit type
        -constants can be declared in any scope - including the global scope
        - constants may only be assigned to a constant expression and not the result of a 
          function call or any value that can be computed at runtime
*/
//constants are valid for the entire time a program runs
const MAX_POINTS: u32 = 100_000;

fn main() {
    //we cannot change the value of this variable as it is immutable - all variables are immutable by default
    let x = 5;
    //we can change the value of this variable as we have given it the mutable trait
    let mut y = 4;
    println!("The value of x is: {}",x);
    println!("The value of x is: {}",y);
    y=6; //this won't work as x is not mutable
    println!("the value of x is: {}", y);

    //SHADOWING
    /*
        You can declare a new variable with the same name as a previous variable - this means the first variable 
        has been shadowed by the second. 
    */
    let z = 5;
    //a new variable z is declared and assigned the value of the old value of z + 1
    let z = z+1;
    //a new variable z is declared and assigned the value of the old value of z * 2
    let z = z*2;
    //this will print ((5)+1)*2=12
    println!("The Value of z is: {}", z);

    /*
        Shadowing is different to the variable being immutable as we'll get a compile-time error if we forget to 
        redeclare the variable (try reassigning the variable without using the let keyword). 
        It allows us to perform transformations on the variable while keeping it immutable after those 
        transformations have been completed.

        The other difference is that because we are creating a new variable, we can change the type of the value,
        but reuse the same name (We saw this in section 2 with the variable 'guess') We see this example below
    */
    //changing the inferred type from a string to a number using shadowing:
    let spaces = "     "; //5 spaces
    let spaces = spaces.len();
    println!("the value of spaces is: {}", spaces);

    //this ability to change the inferred type doesn't work for mutable variables:
    //let mut spaces = "   ";
    //spaces = spaces.len();
}
