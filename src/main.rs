use std::env;
use std::fs;
use std::process;

// this is for creating directory or directories
fn create_directory(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => println!("\nCreated directory: {}\n", path),
        Err(e) => eprintln!("\nError occured while creating directory, {}\n", e),
    };
}

fn remove_direcotory(path: &str) {
    match fs::remove_dir(path) {
        Ok(_) => println!("\nRemoved directory: {}\n", path),
        Err(e) => eprintln!("\nError occured while removing directory, {}\n", e),
    };
}

// fn move_directory() {

// }

// fn copy_directory() {

// }

fn know_current_directory() {
    match env::current_dir() {
        Ok(p) => println!("\nCurrent working directory is: {}\n", p.display()),
        Err(e) => eprintln!(
            "\nError occured while providing current working directory {}\n",
            e
        ),
    };
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    // generate error and end program, if there are less then 3 args, because either path or flag or both not provided.
    if arguments.len() < 3 && arguments[1] != "-p" {
        eprintln!(
            "\noption or path or both are missing, please provide valid option and path to create directory\n"
        );
        process::exit(1);
    }

    // for now i will using flag temporarily will make it paramenent
    let flag = &arguments[1];
    let address = &arguments[2];

    if flag == "-c" {
        create_directory(&address);
    } else if flag == "-r" {
        remove_direcotory(&address);
    } else if flag == "-p" {
        know_current_directory();
    } else {
        println!("\nWrong option is provided, provide valid option\n");
        println!("'-- -c' for creating directory");
        println!("'-- -r' for removing directory");
        println!();
    }
}
