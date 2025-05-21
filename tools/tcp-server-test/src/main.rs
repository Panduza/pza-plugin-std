use serde::Deserialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use std::error::Error;
use tokio::fs::File;


#[derive(Debug, Deserialize)]
struct Config {
    ip: String,
    port: u16,
}

async fn load_config() -> Result<Config, Box<dyn Error>> {
    let file = File::open("config.json").await?;
    let mut reader = BufReader::new(file);
    let mut contents = Vec::new();
    reader.read_to_end(&mut contents).await?;
    let config: Config = serde_json::from_slice(&contents)?;
    Ok(config)
}

async fn listen() -> Result<TcpStream, Box<dyn Error>> {
    let config = load_config().await?;
    let device_addr = format!("{}:{}",config.ip,config.port);

    match TcpListener::bind(&device_addr).await {
        Ok(listener) => {
            println!("Listening on adress {}",&device_addr);
            let (socket, _) = listener.accept().await?;
            return Ok(socket);
        }
        Err(e) => {
            println!("Error : listening failed : {}",e);
            return Err(Box::new(e));
        }
    };
}

async fn run() -> Result<bool, Box<dyn Error>> {
    println!("Starting the server ... ");
    let stream = listen().await?;
    let (mut reader, mut writer) = stream.into_split();
    writer.write_all("reboot succeeded".as_bytes()).await?;
    println!("Server started : waiting for messages");
    let mut buf = [0u8; 1024];
    loop {
        println!("Waiting for message...");
        let bytes = match reader.read(&mut buf).await {
            Ok(0) => {
                println!("Client disconnected.");
                return Ok(true);
            }
            Ok(n) => n,
            Err(e) => {
                println!("Unexpected error: {}", e);
                return Ok(true);
            }
        };

        if bytes == 0 {
            println!("Client disconnected.");
            return Ok(true);
        }

        let msg = String::from_utf8_lossy(&buf[..bytes]).trim().to_string();
        println!("[Message received] {:?}", msg);

        if msg == "reboot" {
            println!("Reboot requested");
            return Ok(true);
        }

        let response = format!("echo : {}", msg);
        writer.write_all(response.as_bytes()).await?;
        println!("[Response] {:?}", response);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let reboot = run().await?;
        if !reboot {
            break;
        }
        println!("Restarting run()...");
    }

    Ok(())
}
