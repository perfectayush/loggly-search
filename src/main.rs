mod loggly;
mod cli;

use loggly::Loggly;

#[tokio::main]
async fn main() {
    let args = cli::get_cli_args();
    let token = args.value_of("token").unwrap_or("unset");
    let account = args.value_of("account").unwrap_or("unset");
    let from = args.value_of("from").unwrap();
    let query = args.value_of("query").unwrap();
    let verbosity = args.occurrences_of("verbosity") as usize;
    let mut loggly_client = Loggly::init(&account, &token, &from, &query);
    stderrlog::new()
        .module(module_path!())
        .verbosity(verbosity)
        .init()
        .unwrap();
    loggly_client.main_loop().await;
}
