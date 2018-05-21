const { spawn } = require('child_process');

let done = false

const rust_process = spawn(
  'target/debug/monitor-poc',
  {
    stdio: [
      'pipe',  // stdin
      'pipe',  // stdout
      'ignore' // stderr
    ]
  }
);

rust_process.on('error', (err) => {
  console.log(`Failed to start monitor-poc: ${err}`);
});

rust_process.stdout.on('data', (data) => {
  // Slice removes trailing \n from the Buffer
  console.log(`from child: "${data.slice(0, -1)}"`);
});

rust_process.on('exit', (code) => {
  console.log(`child process exited with code ${code}`);
  done = true;
});

let iter = 0;

const intervalId = setInterval(() => {
  if (!done) {
    if (iter == 10) {
      rust_process.stdin.end("That's all folks\n");
    } else {
      rust_process.stdin.write(`${iter}\n`);
    }
  } else {
    console.log("(parent: Bye!)");
    clearInterval(intervalId);
  }

  iter++;
}, 100);

console.log("(parent: main thread ended)");
