use clap::{App, Arg, ArgMatches, crate_version};

pub fn get_cli_args() -> ArgMatches<'static> {
    let matches = App::new("loggly")
        .version(crate_version!())
        .author("Ayush Goyal <perfectayush@gmail.com>")
        .about("Search logs from loggly cloud")
        .arg(Arg::with_name("account")
             .short("a")
             .long("account")
             .value_name("ACCOUNT")
             .env("LOGGLY_ACCOUNT")
             .help("Loggly Account to search logs in use")
             .takes_value(true))
        .arg(Arg::with_name("token")
                .short("t")
                .long("token")
                .value_name("API_TOKEN")
                .env("LOGGLY_API_TOKEN")
                .help("Loggly API token to use")
                .takes_value(true))
        .arg(Arg::with_name("from")
             .short("f")
             .long("from")
             .value_name("FROM")
             .default_value("-10m")
             .allow_hyphen_values(true)
             .help(
"Time to start seaching from.
Can take absolute date in format 'yyyy-MM-dd HH:mm:ss.SSS'. Eg: `2020-08-13T18:45:00.000`
Or relative date values like `now`, last 10 mins `-10m`, last 5 weeks `-5w`, last 12 hours `-12h`
Or absolute Unix timestamp in *milliseconds* eg. `1617315836793`"
             )
            .takes_value(true))
        .arg(Arg::with_name("query")
             .short("q")
             .long("query")
             .value_name("Query")
             .default_value("*")
             .allow_hyphen_values(true)
             .help(
                 "Query search. Refer: https://documentation.solarwinds.com/en/Success_Center/loggly/Content/admin/search-query-language.htm "
             )
             .takes_value(true))
        .arg(Arg::with_name("verbosity")
             .short("v")
             .multiple(true)
             .help("Increase message verbosity"))
        .get_matches();
    matches
}
