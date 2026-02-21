# stdio piping

Exploring how os's use file descriptors and pipes to point stdin/out to child processes. 

I was curious how this worked after being exposed to claude code's stdio mcp server. 

This example here spawns a server (the mcp server in the CC example) and pipes both stdin/out to the current parent process. From there, I write to the childs file descriptor 0 (stdin) and read from childs file descriptor 1 (stdout).


## Running it
```
cargo run -- client
```

Type anything and hit enter to see it echoed back. Type `end` to quit.
