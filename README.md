# Integrating Node Js and Rust for a long running monitor application

This is a proof of concept to see how a NodeJS parent application could pipe an infinite stream of
data from a Rust process which pretends to be a conduit for data from a serial device.

## Requirements

* The solution must work on Windows and Unix OSes.
* The child process must continuously stream data to the parent until signaled to stop.
* The parent process must not block it's main thread while data is being received.
* The parent process must be able to signal the child process to stop, cleanly.

## Solution

Everything can be achieved using `stdio` streams, and piping between parent and child.

Using NodeJS's [child_process.spawn](https://nodejs.org/api/child_process.html#child_process_child_process_spawn_command_args_options)
allows for streaming of data from the child asynchronously. The main thread does not get blocked.

Signaling the child process to stop can be done trivially by closing the child process's `stdin`
stream within the parent (`child.stdin.end()`).

After starting up a thread streaming data back to the parent, the child process blocks on a `read`
to it's `stdin`. When the stream receives the `EOF` (End Of File) marker, it shuts down. When the
parent process closes the stream this sends `EOF` to the child, triggering a clean shutdown
procedure.

## Running

* Install Rust 1.25 or later
* Install NodeJS 10+

Then:

```
$ cargo build && node main.js
```

Example output:

```
(parent: main thread ended)
from child: "Pretend serial output 0"
from child: "Pretend serial output 1"
from child: "Pretend serial output 2"
from child: "Pretend serial output 3"
from child: "Pretend serial output 4"
from child: "Pretend serial output 5"
from child: "Pretend serial output 6"
from child: "Pretend serial output 7"
from child: "Pretend serial output 8"
from child: "Pretend serial output 9"
(parent: closing child process stdin)
from child: "(rust: Got EOF)
(rust: sending stop message)
(rust: waiting for service stop ...)"
child process exited with code 0
```
