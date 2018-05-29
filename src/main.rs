use std::io::{self, Read};
use std::sync::mpsc::{channel, Sender, TryRecvError};
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
    let (send_stop, thread_handle) = pretend_serial_service();

    // Block awaiting any data or the stream being closed
    let mut in_buffer = []; // empty, we will not actually read anything
    match io::stdin().read(&mut in_buffer) {
        Ok(_) => println!("(rust: Got EOF)"),
        Err(e) => return Err(e.into()),
    }

    println!("(rust: sending stop message)");
    send_stop.send(())?;

    println!("(rust: waiting for service stop ...)");
    thread_handle.join().map_err(|_| MyError::ThreadPanicked)
}

fn pretend_serial_service() -> (Sender<()>, thread::JoinHandle<()>) {
    let (send_stop, recv_stop) = channel::<()>();

    let handle = thread::spawn(move || {
        for i in 0.. {
            println!("Pretend serial output {}", i);
            sleep(500);

            match recv_stop.try_recv() {
                Ok(_) => break,
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => panic!("recv_stop disconnected"),
            }
        }
    });

    (send_stop, handle)
}

fn sleep(dur_ms: u64) {
    thread::sleep(Duration::from_millis(dur_ms));
}
