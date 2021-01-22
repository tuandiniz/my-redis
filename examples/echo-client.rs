use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()>{

    let socket = TcpStream::connect("127.0.0.1:6349").await?;
    let (mut reader, mut writer) = socket.into_split();

    let write_task = tokio::spawn(async move {

        writer.write_all(b"hello\r\n").await?;
        writer.write_all(b"World\r\n").await?;

        Ok::<_, io::Error>(())
    });

    let mut buff:Vec<u8> = vec![0; 128];
    loop {
        let n = reader.read(&mut buff).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", &buff[..n]);
        println!("GOT String \"{}\"", String::from_utf8(buff[..n].to_vec()).unwrap());
    }

    Ok(())
}