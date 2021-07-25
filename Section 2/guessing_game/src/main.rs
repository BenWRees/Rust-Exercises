/*
    bring the io library into scope which comes from the standard library (std).
    use keyword is the same as the preprocessor command "#include" in C/C++ or "import" in
    Java/Python etc
*/
use std::io;
//brings the Rng library into scope that defines methods that random number generators implement
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    /*
        we create an immutable random number called secret_number
        
        rand::thread_rng() gives us the particular random number generator - the number 
        generator that is local to the current thread of execution and seeded by the 
        operating system
    */
    let secret_number = rand::thread_rng()
        /*
            gen_range this method is defined by the Rng trait brought into scope on line 8. This
            method takes a range expression as an argument and generates a random number in
            the range. 

            Range expressions take the form start,step..end and is exclusive only on the upper
            bound. Can make the upper bound exclusive by sayin start...=end
        */
        .gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess: ");

    /*
        let - creates a new variable
        mut - trait that makes our newly initialised variable be mutable = variables in rust, by
        default, are immutable
        String::new() - static function that calls a new instance of a String from the String
        Library in io::String
        :: - scope operator, in line 6 this is <Library>::<SubLibrary> in the case below this
        is because new is an associated function of String
        'associated method' - implemented on a type rather than an instance
    */

    let mut guess = String::new();

    /*
        if "use std::io" wasn't included we would have to write "std::io::stdin()"
        stdin returns an instance of io::Stdin - a tupe that represents a handle to the
        standard input of the terminal
    */
    io::stdin()
        /*
            We call read_line on the standard input handle (an instance of io::Stdin) to get
            an input from the user. The job of read_line is to take the users input and append
            that to a string (without overwrite) called in the function's argument. The string
            argument needs to be mutable.
            & is a reference which points to where the value of the variable is stored. This is
            a call by reference which allows multiple different function calls access the same
            piece of data without needing to copy the data = this is much better for dealing
            with shared resources in concurrency.
            read_line returns an instance of io::Result a specific version of the generic
            std::Result. Result is an Enumeration (type with a fixed set of values called
            variants)
        */
        .read_line(&mut guess)
        /*
            io::Result allows us to encode error handling information, it's an enum with
            variants: 'Ok' and 'Err'. 'Err' will contain info about how the operation failed.
            The "expect" method causes the program to crash and and display the message that passed
            as an argument to "expect" if io::Result returns an Err value; if io:Result returns
            an Ok value then "expect" will return the value that Ok is holding.
        */
        .expect("Failed to read line");
    /*
        we can use positional arguments in the print line macro as such:
    */
    println!("You guessed: {}", guess);
}
