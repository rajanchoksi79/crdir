use std::fs;
use std::env;
use std::process;

// this is for creating directory or directories
fn create_directory(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => println!("created directory: {}", path),
        Err(e) => eprintln!("Error occured while creating directory, {}", e),
    };
}

// fn remove_direcotory() {

// }

// fn move_directory() {

// }

// fn copy_directory() {

// }

fn main() {
    let arguments: Vec<String> = env::args().collect();
    
    if arguments.len() < 3 {
        eprintln!("option or path or both are missing, please provide valid option and path to create directory");
        process::exit(1);        
    }

    // for now i will using flag temporarily will make it paramenent
    let flag = &arguments[1];
    let address = &arguments[2];

    if flag == "-c" {
        create_directory(&address);
    }
    else {
        println!("wrong option is provided, provide valid option");
        println!("'-- -c' for creating directory");
    }
}
