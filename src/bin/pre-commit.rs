use std::process;

fn main(){
    let exit;

    let test_command = process::Command::new("cargo")
        .arg("test")
        .status();

    match test_command {
        Ok(status) => {
            exit = status.code().unwrap() as i32;
        },
        Err(err) => {
            eprint!("Error when Running the tests {:?}", err);
            exit = 1;
        }
    }

    // Run clippy
    process::Command::new("cargo")
        .arg("clippy")
        .status()
        .unwrap_or_else( |err| {
            eprint!("Error running clippy {:?}", err);
            process::exit(1);
        });

    process::exit(exit);
}