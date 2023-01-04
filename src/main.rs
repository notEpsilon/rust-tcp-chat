use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel::<String>(32);

    loop {
        let (mut conn, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = conn.split();
            
            let mut buffer = BufReader::new(reader);
            let mut line = String::new();
        
            loop {
                tokio::select! {
                    result = buffer.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                    }
                    result = rx.recv() => {
                        writer.write_all(&result.unwrap().as_bytes()).await.unwrap();
                        line.clear();
                    }
                }
            }
        });
    }
}
