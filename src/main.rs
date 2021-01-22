use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};

type Db = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;
const SHARDS: usize = 6;

fn hash<T: Hash>(t: &T) -> usize {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    (s.finish() as u64).try_into().unwrap()
}

#[tokio::main]
async fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let mut db:Vec<Mutex<HashMap<String, Bytes>>> = Vec::with_capacity(SHARDS);
    
    for _ in 0..SHARDS {
        db.push(Mutex::new(HashMap::new()));
    }

    let db:Db = Arc::new(db);

    println!("Listening...");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db_clone = db.clone();

        tokio::spawn (async move {
            process(socket, db_clone).await;
        });
    }
}

async fn process(stream: TcpStream, db: Db) {
    
    let mut connection = Connection::new(stream);    

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let db = &db[hash(&cmd.key()) % SHARDS];
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let db = &db[hash(&cmd.key()) % SHARDS];
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented {:?}", cmd)
        };
        
        connection.write_frame(&response).await.unwrap();
    }
}