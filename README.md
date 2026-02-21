# ioexperiments

A small Rust project I wrote to learn and explore how operating systems handle piping between processes.

The single binary acts as both a client and a server. The client spawns the server as a child process, hooks up stdin/stdout pipes between them, and passes messages back and forth. The server just echoes back whatever you type, until you send "end".

## What I learned

- When a process's stdout is piped (not connected to a terminal), it switches from line-buffered to fully-buffered. This means you need to explicitly `flush()` after writes or the data just sits in a buffer and the other side hangs waiting.
- Rust's `{:?}` debug formatter escapes newlines in strings, which can silently eat the newline that terminal line-buffering depends on to flush output. Tricky.
- `Command::spawn()` gives you owned handles for the child's stdin/stdout, but you need to `.take()` them out of the `Child` struct to use them independently.

## Running it

```
cargo build
./target/debug/ioexperiments client
```

Type anything and hit enter to see it echoed back. Type `end` to quit.
