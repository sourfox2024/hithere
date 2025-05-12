use std::io;

fn main() {
    println!("What is your name?");
    // Let's variable 'name' become mutable
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
                    //trims the string
                    let trim1 = name.trim();
            println!("Hi {}, nice to meet you.", trim1);
        },
        Err(error) => println!("error: {error}"),
    }
}

