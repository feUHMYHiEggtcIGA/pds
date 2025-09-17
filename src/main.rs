use tokio;

use pds::lgc::scrnr;

#[tokio::main]
async fn main() {
    println!("start");
    loop {
        match scrnr().await {
            Err(v) => println!("err: {:?}", v),
            _ => {},
        }
    }
}
