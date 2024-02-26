use std::io::{Write, stdin, stdout};

fn main() {
    loop {
        print!("user> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(0) => return,
            Ok(_) => (),
            Err(_) => println!("Error reading line")
        }

        println!("{}", line)
    }
}
