use clap::{App, Arg, ArgMatches};

pub fn get_cli_args() -> ArgMatches<'static> {
    let matches = App::new("loggly")
        .version("1.0")
        .author("Ayush Goyal <perfectayush@gmail.com>")
        .about("Search logs from loggly cloud")
        .arg(Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("API_TOKEN")
                .env("LOGGLY_API_TOKEN")
                .help("Loggly API token to use")
                .takes_value(true))
        .arg(Arg::with_name("account")
             .short("a")
             .long("account")
             .value_name("ACCOUNT")
             .env("LOGGLY_ACCOUNT")
             .help("Loggly Account to search logs in use")
             .takes_value(true)).get_matches();
    matches
}
