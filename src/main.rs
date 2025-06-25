use std::env;
use std::fs;
use std::process;

// this is for creating directory or directories, if there is no directory in chain then generate error
fn create_directory(path: &str) {
    match fs::create_dir(path) {
        Ok(_) => println!("\nCreated directory: {}\n", path),
        Err(e) => eprintln!("\nError occured while creating directory, {}\n", e),
    };
}

// this will create directory recursively, even though there is not directory in chain of provided directory, it will create it.
fn create_directory_recursive(path: &str) {
    match fs::create_dir_all(path) {
        Ok(_) => println!("\nCreated directory recursively: {}\n", path),
        Err(e) => eprintln!(
            "\nError occured while creating directory recursively, {}\n",
            e
        ),
    };
}

// this will remove empty directory
fn remove_direcotory(path: &str) {
    match fs::remove_dir(path) {
        Ok(_) => println!("\nRemoved directory: {}\n", path),
        Err(e) => eprintln!("\nError occured while removing directory, {}\n", e),
    };
}

// this will remove any directory even when it has content inside.
fn remove_direcotory_recursive(path: &str) {
    match fs::remove_dir_all(path) {
        Ok(_) => println!("\nRemoved directory recursively: {}\n", path),
        Err(e) => eprintln!(
            "\nError occured while removing directory recursively, {}\n",
            e
        ),
    };
}

// just like move this is also temp things, i will have write real copy code, so will do later, after few tests on these basic things.
fn copy_directory(old_path: &str, new_path: &str) {
    if old_path != new_path {
        match fs::create_dir(new_path) {
            Ok(_) => println!("\nCopied directory to new path: {}\n", new_path),
            Err(e) => eprintln!("\nError occured while copying directory, {}\n", e),
        }
    } else {
        println!(
            "Both of the given path are the same, please provide different paths to copy directory to one path to another"
        );
    }
}

// this is temporary here that i removed directory from old path and created new in new path, just an illution, but will have to really move directories, so will do it later.
fn move_directory(old_path: &str, new_path: &str) {
    if old_path != new_path {
        match fs::remove_dir(old_path) {
            Ok(_) => println!("\nMoving Directory from old path: {}", old_path),
            Err(e) => eprintln!("\nError occured while moving directory, {}\n", e),
        }

        match fs::create_dir(new_path) {
            Ok(_) => println!("\nDirectory moved to new path: {}\n", new_path),
            Err(e) => eprintln!("\nError occured while moving directory, {}\n", e),
        }
    } else {
        println!(
            "Both of the given path are the same, please provide different paths to move directory to one path to another"
        );
    }
}

// this is for knowing current path, that we are in.
fn know_current_directory() {
    match env::current_dir() {
        Ok(p) => println!("\nCurrent working directory is: {}\n", p.display()),
        Err(e) => eprintln!(
            "\nError occured while providing current working directory {}\n",
            e
        ),
    };
}

fn rename_dir(old_path: &str, new_path: &str) {
    if old_path != new_path {
        match fs::rename(old_path, new_path) {
            Ok(_) => println!(
                "\nDirectory name changed from: {}, to: {}\n",
                old_path, new_path
            ),
            Err(e) => eprintln!("\nError occured while renaming the directory, {}\n", e),
        };
    } else {
        eprintln!("Both of the given path are the same with same directory name, please provide different directory name rename it");
    }
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
    let mut address: &str = "";
    if arguments.len() > 2 {
        address = &arguments[2];
    }

    let mut address_two: &str = "";
    if arguments.len() > 3 {
        address_two = &arguments[3];
    }

    if flag == "-c" {
        create_directory(&address);
    } else if flag == "-m" {
        move_directory(&address, &address_two);
    } else if flag == "-o" {
        copy_directory(&address, &address_two);
    } else if flag == "-r" {
        remove_direcotory(&address);
    } else if flag == "-u" {
        create_directory_recursive(&address);
    } else if flag == "-v" {
        remove_direcotory_recursive(&address);
    } else if flag == "-n" {
        rename_dir(&address, &address_two);
    } else if flag == "-p" {
        know_current_directory();
    } else {
        println!("\nWrong option is provided, provide valid option\n");
        println!("'-- -c' for creating directory");
        println!("'-- -r' for removing directory");
        println!("'-- -m' for moving directory");
        println!("'-- -o' for copying directory");
        println!("'-- -p' for knowing directory");
        println!();
    }
}
