use std::{
    env::args,
    io::{self, BufRead, Write}, process::Stdio,
};

use std::process::Command;

#[derive(Debug)]
enum ClientOrServer {
    Client,
    Server,
}

fn main() {
    let arg = match args().nth(1).unwrap().as_str() {
        "client" => ClientOrServer::Client,
        "server" => ClientOrServer::Server,
        _ => unreachable!(),
    };

    match arg {
        ClientOrServer::Client => {
            let mut server = Command::new(
                "/binary/path/here",
            )
                .arg("server")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let mut local_io_buf = String::new();
            let mut server_stdin = server.stdin.take().unwrap();
            let mut server_stdout = std::io::BufReader::new(server.stdout.take().unwrap());

            loop {
                local_io_buf.clear();
                io::stdin().read_line(&mut local_io_buf).unwrap();

                server_stdin.write_all(local_io_buf.as_bytes()).unwrap();
                server_stdin.flush().unwrap();

                let mut response = String::new();
                server_stdout.read_line(&mut response).unwrap();

                println!("Echo: {:?}", response.trim());

                if local_io_buf.trim_end() == "end" {
                    break;
                }
            }
        }
        ClientOrServer::Server => {
            let mut buffer = String::new();
            let mut stdout = io::stdout();

            loop {
                buffer.clear();
                io::stdin().read_line(&mut buffer).unwrap();
                let input = buffer.trim_end();

                if input == "end" {
                    stdout.write(b"Hit end here").unwrap();
                    stdout.flush().unwrap();
                    break;
                } 
                else {
                    stdout.write(buffer.as_bytes()).unwrap();
                    stdout.flush().unwrap();
                }
            }
        }
    }
}
