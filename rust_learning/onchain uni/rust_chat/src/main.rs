use tokio::net::TcpListener;
use futures_util::stream::{StreamExt, TryStreamExt};
use futures_util::sink::SinkExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on port 8080");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let addr = stream.peer_addr().unwrap();
        println!("Connection established: {}", addr);

        tokio::spawn(async move {
            let mut websocket = tokio_tungstenite::accept_async(stream).await.unwrap();

            loop {
                match websocket.try_next().await {
                    Ok(Some(message)) => {
                        if message.is_text() || message.is_binary() {
                            // Wrap the single message in a stream and send it all
                            let mut single_message = futures_util::stream::once(async { Ok(message) });
                            websocket.send_all(&mut single_message).await.unwrap();
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
