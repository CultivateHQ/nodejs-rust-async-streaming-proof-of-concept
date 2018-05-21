use std::io::{self, Read};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

mod errors;
use errors::MyError;

fn main() {
    ::std::process::exit(match run_app() {
       Ok(_) => 0,
       Err(err) => {
           println!("error: {:?}", err);
           1
       }
    });
}

fn run_app() -> Result<(), MyError> {
    let (send_stop, recv_stopped) = pretend_serial_service();

    // Block awaiting any data or the stream being closed
    let mut in_buffer = []; // empty, we will not actually read anything
    match io::stdin().read(&mut in_buffer) {
        Ok(_) => println!("(rust: Got EOF)"),
        Err(e) => return Err(e.into()),
    }

    println!("(rust: sending stop message)");
    send_stop.send(())?;

    println!("(rust: waiting for service stop ...)");
    recv_stopped.recv_timeout(Duration::from_secs(2))?;

    Ok(())
}

fn pretend_serial_service() -> (Sender<()>, Receiver<()>) {
    let (send_stop, recv_stop) = channel::<()>();
    let (send_stopped, recv_stopped) = channel::<()>();

    thread::spawn(move || {
        for i in 0.. {
            println!("Pretend serial output {}", i);
            sleep(500);

            match recv_stop.try_recv() {
                Ok(_) => {
                    send_stopped.send(()).expect("main disconnected too quickly");
                    break;
                },
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => panic!("recv_stop disconnected"),
            }
        }
    });

    (send_stop, recv_stopped)
}

fn sleep(dur_ms: u64) {
    thread::sleep(Duration::from_millis(dur_ms));
}
