/// Load string contents of a file on the hard drive.
/// This function will output an error if it failed to delete the file.
pub(super) fn delete_file(path: &String) -> Result<(), String> {
    //Try do delete 
    match std::fs::remove_file(path) {
        Result::Ok(_) => Result::Ok(()),
        Result::Err(e) => Result::Err(format!("The file {} could not be deleted. Reason: {}", path, e))
    }
}

/// Save string contents to a file of path.
/// This will create the file if it does not exist or completely rewrite if it does.
/// This function will output an error if it failed to save the file.
pub(super) fn save_file(path: &String, contents: &String) -> Result<(), String>{
    match std::fs::write(path, contents) {
        Result::Ok(_) => Result::Ok(()),
        Result::Err(e) => Result::Err(format!("Unable to write file {}. Reason: {}", path, e))
    }
}

/// Load string contents of a file on the hard drive.
/// This function will output an error if it failed to load the file.
pub(super) fn load_file(path: &String) -> Result<String, String> {
    match std::fs::read_to_string(path) {
        Result::Ok(result) => Result::Ok(result),
        Result::Err(e) => Result::Err(format!("Unable to read file {}. Reason: {}", path, e))
    }
}

/// Checks if the provided path is valid.
pub(super) fn does_file_exist(path: &String) -> bool {
    std::path::Path::new(path).exists()
}