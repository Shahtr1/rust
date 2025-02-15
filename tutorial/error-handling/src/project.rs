use std::{
    fs::write,
    io::{stdin, Result},
    process,
};

pub fn run() {
    match write_to_file() {
        Ok(filename) => println!(
            "Successfully
wrote to file {}",
            filename
        ),
        Err(error) => {
            println!(
                "There was an error:
{error}"
            );
            process::exit(1);
        }
    }
}

fn write_to_file() -> Result<String> {
    let mut file_name = String::new();
    let mut contents = String::new();

    println!("What file would you like to write to?");
    stdin().read_line(&mut file_name)?;

    println!("What would you like to write to the file?");
    stdin().read_line(&mut contents)?;

    write(file_name.trim(), contents.trim())?;

    Ok(file_name)
}
