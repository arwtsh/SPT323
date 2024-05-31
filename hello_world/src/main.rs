//fn declares a function. The main function is the entry point of the program.
fn main() {
    //for makes a block of code iterate from the first number (inclusive) to the second number (exclusive).
    for i in 0..4 {
        //If checks a boolean condition, and if it evaluates to true, executes the following block of code.
        if i % 2 == 0 {
            println!("Hello, world!"); //Print to the console with a new line.
        }
        //else runs a block of code if the previous if statement was false.
        else {
            println!("Goodbye, world!"); //Print to the console with a new line.
        }
    }
}
