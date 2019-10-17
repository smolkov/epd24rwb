//! EPD Dashboard driver
//!
//! To send messages, do:
//!
//! ```sh
//! $ nc -u localhost 8080
//! ```
//!
use futures::{
    channel::mpsc,
    SinkExt,
};

use async_std::io;
use async_std::net::UdpSocket;
use async_std::task;

use epd24rwb::*;

async fn brocker(mut sender: Sender<String>) -> io::Result<()> {
        let socket = UdpSocket::bind("127.0.0.1:8088").await?;
        let mut buf = vec![0u8; 1024];

        println!("Listening on {}", socket.local_addr()?);

        loop {
            let (n, peer) = socket.recv_from(&mut buf).await?;
            let msg: String = String::from_utf8_lossy(&buf).to_string();
            sender.send(msg).await.unwrap();
            let sent = socket.send_to(&buf[..n], &peer).await?;
            println!("Sent {} out of {} bytes to {}", sent, n, peer);
        }
}
async fn try_main() -> io::Result<()> {
    femme::start(log::LevelFilter::Trace).unwrap();
    log::info!("spawn udp brocker ");
    let config  = config::read(".");
    let (sender, receiver) = mpsc::unbounded();
    let brocker = task::spawn(brocker(sender));
    log::info!("spawn dashboard");
    let dashboard = task::spawn(start(config.await?,receiver));

    dashboard.await?;
    brocker.await?;
    Ok(())
}
fn main() -> io::Result<()> {
    task::block_on(try_main())
}
