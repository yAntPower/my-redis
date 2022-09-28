use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
}
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manage = tokio::spawn(async move{
        let mut conn = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(msg) = rx.recv().await {
            match msg{
                Command::Set { key, val, resp }=>{
                    let res = conn.set(&key, val).await;
                    resp.send(res).unwrap();
                },
                Command::Get { key, resp }=>{
                    let res = conn.get(&key).await;
                    resp.send(res).unwrap();
                },
            }
        }

    });
    let task1 = tokio::spawn(async move{
        let (tx_one,rx_one) = oneshot::channel();
        let msg = Command::Set { key: "antPower".to_string(), val: "learn".into(), resp: tx_one };
        if tx.send(msg).await.is_err(){
            panic!("task1 send err");
        }
        let res = rx_one.await;
        println!("task1 Result :{:?}",res);
    });
    let task2 = tokio::spawn(async move{
        let (tx_one,rx_one) = oneshot::channel();
        let msg = Command::Get { key: "antPower".to_string(),resp: tx_one };
        if tx2.send(msg).await.is_err(){
            panic!("task2 send err");
        }
        let res = rx_one.await;
        println!("task2 Result :{:?}",res); 
    });

    task1.await.unwrap();
    task2.await.unwrap();
    manage.await.unwrap();
}
