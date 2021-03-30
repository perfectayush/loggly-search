mod loggly;
mod cli;


#[tokio::main]
async fn main() {
    // println!("{:?}", res.get("next"));
    let args = cli::get_cli_args();
    let token = args.value_of("token").unwrap_or("unset");
    let account = args.value_of("account").unwrap_or("unset");
    let res = loggly::fetch_loggly_logs(&account, &token).await.unwrap();
    println!("{:?}", res.get("next"));

}
