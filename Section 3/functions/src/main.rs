/*
    statement: instructions that perform some action and don't return a value
    expression: evaluate to a resulting value

    expressions do not include ending semicolons 
*/

fn main() {
    println!("Hello, world!");
    another_function();
    even_another_function(5);

    //this is a statement - an instruction that performs some action and doesn't return a value
    let y = 3;

    //let x = (let y = 3); - this won't work as the let statement does not return a value 
    //assignments don't return values like in C 

    //expressions can be 3+1 which evaluates to the integer 4, calling a function is an expression
    let x = {
        //this block acts as an expression - almost like an anonymous function which assigns y to 3 and 
        //returns the expression x+1 = 4
        let y = 3;
        y+1
    };

    println!("The value of x is: {}", x);

    let w = plus_one(5);
    println!("the value of w is: {}", w);



}

//a defined function - the function definition is a statement and not an expression
fn another_function() {
    println!("Another Function");
}

//in function sigs you must declare the type of the parameter with type annotation
fn even_another_function(x: i32) {
    println!("The value of e is: {}", x);
}

//a function that returns a value - the function must end with an expression to return a value
fn plus_one(x: i32) -> i32 {
    x+1
}
