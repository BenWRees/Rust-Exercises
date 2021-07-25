/*
    bring the io library into scope which comes from the standard library (std).
    use keyword is the same as the preprocessor command "#include" in C/C++ or "import" in
    Java/Python etc
*/
use std::io;
//brings the Rng library into scope that defines methods that random number generators implement
use rand::Rng;
//brings the Ordering library into scope from the standard library that allows to define a 
//trichotomy on a poset
use std::cmp::Ordering;

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


    loop {
        println!("Please input your guess: ");

        /*
            let - creates a new variable
            mut - trait that makes our newly initialised variable be mutable = variables in rust, 
            by default, are immutable
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
                an input from the user. The job of read_line is to take the users input and 
                append that to a string (without overwrite) called in the function's argument. 
                The string argument needs to be mutable.
                & is a reference which points to where the value of the variable is stored. 
                This is a call by reference which allows multiple different function calls 
                access the same piece of data without needing to copy the data = this is 
                much better for dealing with shared resources in concurrency.
                read_line returns an instance of io::Result a specific version of the generic
                std::Result. Result is an Enumeration (type with a fixed set of values called
                variants)
            */
            .read_line(&mut guess)
            /*
                io::Result allows us to encode error handling information, it's an enum with
                variants: 'Ok' and 'Err'. 'Err' will contain info about how the operation failed.
                The "expect" method causes the program to crash and and display the message that 
                passed as an argument to "expect" if io::Result returns an Err value; if 
                io:Result returns an Ok value then "expect" will return the value that Ok is 
                holding.
            */
            .expect("Failed to read line");

        /*
            As Rust is type inferred but still strong, static typing we are forced to sometimes 
            explicitely define the type of a variable. guess was before inferred to be a String 
            based on its initialisation, however cmp cannot compare a string and a number type,
            so guess must be of the same type as secret_number. So we create a new guess that is
            and unsigned 32 bit number, secret_number is i32 as it is not explicitly defined 
            (secret_number's type will be inferred to be u32 due to annotating guess to be u32)

            We can shadow the string guess by declaring a variable with the same name.

            the method trim on a string instance will eliminate any whitespaces 
            at the beginning and the end of the string literal (gets rid of the \n whitespace)

        */
        //the colon after guess annotates the variables type and allows us to explicitly 
        //define the variables type
        //change expect method for a match to go from throwing an error to error handling
        let guess: u32 = match guess.trim()
            //parse method takes a string instance and turns it into a number type 
            .parse() {
                //can explicitly tell the Result instance to handle each variant
                //if Result is Ok then we just return its value
                Ok(num) => num,
                //if Result is Err then we go to the next iteration of the loop and ask for 
                //another guess
                Err(_) => continue,
            }; // have to end guess with a semicolon
            /*
                parse could cause an error, such that the ANSI characters have no way to 
                convert to a number. As parse could fail it returns a Result type so we can 
                handle the Result variants with the expect method.
            */
            //.expect("Please type a number!");
        
        /*
            we can use positional arguments in the print line macro as such:
        */
        println!("You guessed: {}", guess);

        /*
            the method cmp compares two numbers and returns an instance of the enumeration 
            Ordering so will return one of the following variants: {Less,Greater,Equal}.

            We use a match expression to decide what to do next based on what variant of 
            cmp::Ordering is returned by cmp.

            A match expression is made up of arms, each arm consisting of a pattern and the code
            that should be run if the value given to the beginning of the match expression fits 
            that arm's pattern. = This is much like a guard in Haskell.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                //when cmp returns the variant "Equal" - print "You Win!" and break out of 
                //infinite loop
                println!("You Win!");
                break;
            }
        }
    }
    

}
