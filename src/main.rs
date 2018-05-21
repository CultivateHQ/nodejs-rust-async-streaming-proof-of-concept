use std::io::{self, BufRead};

fn main() {
    ::std::process::exit(match run_app() {
       Ok(_) => 0,
       Err(err) => {
           println!("error: {:?}", err);
           1
       }
    });
}

fn run_app() -> io::Result<()> {
    let stdin = io::stdin();
    let mut in_handle = stdin.lock();
    let mut in_buffer = String::new();

    loop {
        in_buffer.clear();
        match in_handle.read_line(&mut in_buffer) {
            Ok(0) => {
                println!("(rust: Got EOF)");
                break;
            },
            Ok(_bytes_read) => print!("from parent: {}", in_buffer),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
