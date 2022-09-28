use mini_redis::{client, Result};
#[tokio::main]
async fn main() -> Result<()> {//为了方便使用“?”操作符所以添加一个Result返回，否则需要写 if let 或者match
    //链接redis服务端
    let mut client = client::connect("127.0.0.1:6379").await?;
    println!("set start");
    //设置key和value
    client.set("antPower", "work hard".into()).await?;
    println!("set over");
    //获取value
    let val = client.get("antPower").await?;
    println!("从服务器端获取到结果={:?}",val);
    Ok(())
}