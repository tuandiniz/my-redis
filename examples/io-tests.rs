use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tokio::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 20];

    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);
    println!("The content: {}", String::from_utf8(buffer[..n].to_vec()).unwrap());
    Ok(())
}