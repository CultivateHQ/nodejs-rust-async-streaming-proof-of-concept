const { spawn } = require('child_process');

const rust_process = spawn(
  'target/debug/monitor-poc'
);

rust_process.on('error', (err) => {
  console.log(`Failed to start monitor-poc: ${err}`);
});

rust_process.stdout.on('data', (data) => {
  console.log(`from child: "${data.toString().trimRight()}"`);
});

rust_process.stderr.on('data', (data) => {
  console.log(`from child (stderr): "${data.toString().trimRight()}"`);
});

rust_process.on('exit', (code) => {
  console.log(`child process exited with code ${code}`);
});

setTimeout(() => {
  console.log("(parent: closing child process stdin)");
  rust_process.stdin.end();
}, 5000);

console.log("(parent: main thread ended)");
