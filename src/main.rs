// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;

mod cache;
mod db;
mod ui;

fn main() {}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//
//     loop {
//         let (mut socket, _) = listener.accept().await?;
//
//         tokio::spawn(async move {
//             let mut buf = [0; 1024];
//
//             // Read data from the socket and write the data back
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     Ok(0) => return,
//                     Ok(n) => n,
//                     Err(e) => {
//                         eprint!("Failed to read from socket; err = {:?}", e);
//                         return;
//                     }
//                 };
//
//                 // Write the data back
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }
//             }
//         });
//     }
// }
