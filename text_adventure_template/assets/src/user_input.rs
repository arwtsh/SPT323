/// Read in user input from terminal.
pub fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read user input."); //Read user input from terminal.
    user_input.trim().to_string()
}