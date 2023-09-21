use std::net::SocketAddr;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::mpsc,
};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
#[non_exhaustive]
enum Message {
    Bottle(String),
    Letter {
        from: String,
        to: String,
        content: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[derive(Debug)]
    struct Message {
        from: Option<String>,
        to: String,
        content: String,
    }
    impl std::fmt::Display for Message {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let from = self.from.as_deref().unwrap_or("nobody");
            write!(
                f,
                "Message from \"{}\" to \"{}\": \"{}\"",
                from, self.to, self.content
            )
        }
    }

    let addr = "127.0.0.1:8080"
        .parse::<SocketAddr>()
        .context("Invalid ip:port")?;

    let listener = TcpListener::bind(addr)
        .await
        .context(format!("Failed to bind on {addr}"))?;

    let (tx, mut rx) = mpsc::channel(16);

    let _handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("{msg:?}");
        }
    });

    // marker-start:server_cliche_loop
    loop {
        // marker-start:mutable_binding_in_destructuring_pattern
        let (mut socket, addr) = listener
            .accept()
            .await
            .context("Failed to accept on socket")?;
        // marker-end:mutable_binding_in_destructuring_pattern

        let tx = tx.clone();

        tokio::spawn(async move {
            let (reader, writer) = socket.split();
            handle_connection(addr.to_string(), reader, writer, tx)
                .await
                .expect("Failed to handle connection");
        });
    }
    // marker-end:server_cliche_loop
}

async fn handle_connection<Reader, Writer>(
    name: String,
    reader: Reader,
    mut writer: Writer,
    tx: mpsc::Sender<Message>,
) -> anyhow::Result<()>
where
    Reader: AsyncRead + Unpin,
    Writer: AsyncWrite + Unpin,
{
    let mut line = String::new();
    let mut reader = BufReader::new(reader);

    // marker-start:break_loop_with_value
    loop {
        if let Ok(bytes_read) = reader.read_line(&mut line).await {
            if bytes_read == 0 {
                break Ok(());
            }
            // marker-end:break_loop_with_value
            let msg = serde_json::from_str::<Message>(&line[..bytes_read])
                .unwrap_or_else(|_e| Message::Bottle(line.clone()));
            tx.send(msg)
                .await
                .context("Failed to forward received message")?;
            writer
                .write_all(format!("{name} found your message. Thank you.\r\n").as_bytes())
                .await
                .context("Failed to send bottle notification")?;
        }
        line.clear();
    }
}
