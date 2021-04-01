mod loggly;
mod cli;

use loggly::Loggly;

#[tokio::main]
async fn main() {
    let args = cli::get_cli_args();
    let token = args.value_of("token").unwrap_or("unset");
    let account = args.value_of("account").unwrap_or("unset");
    let mut loggly_client = Loggly::init(&account, &token);
    loggly_client.main_loop().await;
}
