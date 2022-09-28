use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use bytes::Bytes;
use tokio::net::{TcpListener,TcpStream};
use mini_redis::{Connection,Frame};
type Db = Arc<Mutex<HashMap<String,Bytes>>>;
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop{
        let (socket,_ip_info) = listener.accept().await.expect("链接失败！");
        let db = db.clone();
        tokio::spawn(async move{
            process(socket,db).await;
        });
    }
}
async fn process(socket:TcpStream,db:Db){
    use mini_redis::Command::{self,Get,Set};
    let mut conn = Connection::new(socket);
    {
        let mut db = db.lock().unwrap();
        db.insert("aa".to_string(), "bb".into());
    }
    while let Some(frame) = conn.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd)=>{
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                println!("客户端请求：{:?}---HashMap:{:?}",cmd,db);
                // Frame::Simple("Set successded".into())
                Frame::Bulk(cmd.value().clone().into())
            },
            Get(cmd)=>{
                let db = db.lock().unwrap();
                if let Some(val) = db.get(cmd.key()){
                    println!("回复客户端请求1：{:?}",val);
                    Frame::Bulk(val.clone())
                }else {
                    println!("回复客户端请求2：{:?},hashmap:{:?}",cmd,db);
                    Frame::Null
                }
            },
            err=>panic!("错误的命令；{:?}",err),
        };

        conn.write_frame(&response).await.unwrap();
    }
}