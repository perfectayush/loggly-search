extern crate reqwest;
extern crate tokio;

#[tokio::main]
async fn main() {
    let res = client.get("http://google.com")
        .body("the exact body that is sent")
        .send()
        .await;
    print!("{:?}", res);
}
