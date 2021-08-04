use std::io;
fn main() {
    //SCALAR TYPES - represents a single value
        //INTEGER TYPES - number without fractional component
        /*
            We can have signed [i] (can be negative) or unsigned [u] (only positive) and 8-bits,16-bits and 32-bits.
            Signed numbers are stored using two's complement in binary
            Signed variants can store numbers from -(2^(n-1)) to 2^(n-1)-1 for n is the number of bits
            Unsigned variants can store numbers from 0 to 2^(n)-1
        */
        //number literals:
        let dec = 98_222;
        let hex = 0xff;
        let oct = 0o77;
        let bin = 0b1111_0000;
        let byt: u8 = b'A';
        //can have 64 bit but default is 32 bit - i32 is generally the fastest
        /*
            HOW RUST HANDLES INTEGER OVERFLOW:
            If a variable has a value changed outside its storable range then we get an overflow.
            In normal debug mode, Rust includes checks for integer overflow, these overflows will cause 
            the program to panic at runtime = the program exits without an error

            In release mode, Rust doesn't include checks for integer overflow; it instead does two's complement
            wrapping - this allows the program to not panic at runtime but may cause variables to have a value you
            weren't expecting.
            To handle the possibility of overflow we can use families of methods that the std lib provides on 
            primitive numeric types:
                - wrap with wrapping_* methods such as wrapping_add
                - return None if there's an overflow with the checked_* methods
                - return the value and a boolean indicating if there was an overflow with overflowing_* methods
                - saturate at the value's min or max values with saturating_* methods

        */

        //FLOATING-POINT TYPES - f32 and f64 (default is 64 bit). Represented by IEEE-754 standard
        let x = 2.0; //f64 - double precision 
        let y: f32 = 3.0; //f32 - single-precision float

        //NUMERIC OPERATIONS
        //addition
        let sum = 5 + 10;
        
        //subtraction
        let difference = 93.5-4.3;
        
        //multiplication
        let product = 4*30;
        
        //division
        let quotient = 56.7/32.2;

        //modulo
        let modulo = 43 % 5;

        //BOOLEAN TYPES
        let t = true;
        let f: bool = false; //explicit type annotation

        //CHARACTER TYPES - 32-bits in size and is UTF-8 so allows more than just ASCII
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻'; //can assign variables emoji values

        println!("{}",heart_eyed_cat); //terminal will output the emoji

    //COMPOUND TYPES - represents a group of multiple values into one type.
        //TUPLE TYPES
        /*
            general way of grouping together a number of values with a variety of types into one compound type
            tuples have a fixed length - once declared can not grow or shrink in size.
        */
        let tup: (i32, f64, u8) = (500,6.4,1); //explicit type annotation

        //destructuring a tuple: - breaks a tuple into constituent parts
        let tup = (500,6.4,1); 
        let (x,y,z) = tup; //to get individual values out of a tuple we can use pattern matching
        println!("The value of the second element of the tuple is: {0:?}", y);

        //can also destructure by accessing the tuple element directly by a period:
        let x: (i32, f64, u8) = (500,6.4,1);
        let five_hundred = x.0; //accesses the first element of the tuple
        let six_point_four = x.1; //accesses the second element of the tuple
        let one = x.2; //accesses the third element of the tuple

        //ARRAY TYPES - have a fixed length but all elements must be of the same type
        /*
            uses of arrays:
            -when you want your data allocated on the stack than the heap 
            -when you want to ensure you always have a fixed number of elements
        */
        //array isn't as flexible as the vector type (similar collection type that can grow and shrink in size)

        let a = [1,2,3,4,5]; //example of an array
        /*
            a good use of an array over a vector is for storing a variable that needs to know the months of a year 
            or the hours in a day - these are constructs thats are unlikely to add or remove elements dynamically
        */
        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September",
                      "October", "November", "December"];

        let a: [i32; 5] = [1,2,3,4,5]; //explicit type annotation includes the data type in the array and the length

        //can init an array with the same value for each element with the format [value; length]:
        let a = [3; 5]; //init an array that has length 5 and each entry is the value '3'

        //can access elements of an array using indexing as such:
        let a = [1,2,3,4,5];
        let first_elem = a[0];
        let second_elem = a[1];  

        //program that allows us to read the element of the array at an inputted index:
            let a = [1, 2, 3, 4, 5];

            println!("Please enter an array index.");

            let mut index = String::new();

            io::stdin()
                .read_line(&mut index)
                .expect("Failed to read line");

            //usize is architecture length of the unsigned int - on this computer it is 64 bits
            let index: usize = index
                .trim()
                .parse()
                .expect("Index entered was not a number");

            let element = a[index];

            println!("The value of the element at index {} is: {}",index, element);

}
