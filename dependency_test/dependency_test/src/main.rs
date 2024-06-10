//Declare a dependency to standard console input/output.
use std::io;

fn main() {
    println!("\t~~First Day Classes Simulator~~");
    println!("You are a freshmen student attending University College. What is your name?");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read user input.");
    let character_name : String = user_input.trim().parse().expect("Failed to interpret user input as player name.");
    println!("You are {}, it is Sunday night, your very first class begins tomarrow at 9:30.", character_name);

    //Print out first choice
    println!("Do you:");
    println!("1: Go to be early so you are well rested.");
    println!("2: Play games late into the night because you don't have parents to stop you now.");

    //Branch 1
    user_input.clear(); //Clear previous user input.
    io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //read_line appends the input in the console to the end of the user input string.
    let choice_1 : u32 = user_input.trim().parse().expect("Failed to interpret user input.");
    if choice_1 == 1 {
        //Print out result of choice.
        println!("You wake up in plenty time for your Math 101 class. You are able to eat a hearty breakfast in the cafeteria before class.");
        println!("When you arrive at the class, the professor begins a lecture about Mathematics found in nature.");
        println!("\"Euler's Number shows up all over physics, algebra, and geometry,\" the professor says, \"It's somewhat like Pi.\"");
        
        //Question 1_1
        println!("The professor asks you if you know what the symbol for Euler's number.");
        user_input.clear(); //Clear previous user input.
        io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //Get user input
        let euler_symbol : char = user_input.trim().parse().expect("That's incorrect. Euler's number is one English letter."); //Parse user input to just a character
        if euler_symbol == 'e'{ //Check if user is correct
            println!("Well done!");
        } else {
            println!("That's incorrect. The symbol for Euler's number is e.");
        }

        //Question 1_2
        println!("Do you now what the value of e is at 2 decimal places?");
        user_input.clear(); //Clear previous user input.
        io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //Get user input
        let euler_value : f64 = user_input.trim().parse().expect("That's incorrect. Euler's number should only be 3 digits."); //Parse user input to just a float
        if euler_value == 2.72 { //Check if user is correct
            println!("Well done!");
        } else {
            println!("That's incorrect. The value of e is 2.72.");
        }

        println!("After a fun class you retire back to your dorm room for lunch.");
    } else if choice_1 == 2 {
        //Display result of choice
        println!("You stayed up till 4 in the morning playing video games, until you got so tired you went to bed.");
        println!("You slept past your alarm, your phone wasn't nearly loud enough. Now you've missed your class.");
        println!("Do you want to email your professor to apologize? (y/n)");

        //Question 2_1
        user_input.clear(); //Clear previous user input.
        io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //Get user input
        let euler_value : bool = user_input.trim().parse::<char>().expect("Please enter only a y or an n.") == 'y'; //Parse user input to just a bool
        if euler_value{
            println!("You had a rough start, but the professor was understanding and you learned your lesson.");
        } else {
            println!("It's not a great start to the semester. You're turning into a party student.")
        }
    } else {
        //Print out custom error. 
        println!("That is not a valid choice.");
    }

    println!("Thank you for trying out my program.");
}
