mod loggly;
mod cli;

use loggly::Loggly;

#[tokio::main]
async fn main() {
    // println!("{:?}", res.get("next"));
    let args = cli::get_cli_args();
    let token = args.value_of("token").unwrap_or("unset");
    let account = args.value_of("account").unwrap_or("unset");
    let loggly_client = Loggly::init(&account, &token);
    let res = loggly_client.fetch_logs().await.unwrap();
    println!("{:?}", res.get("next"));

}
