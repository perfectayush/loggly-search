# loggly-search

A cli tool to search logs/tail logs from loggly. It fetches events from 
[loggly paginated events API](https://documentation.solarwinds.com/en/Success_Center/loggly/Content/admin/paginating-event-retrieval-api.htm)

Each line is a json event from the paginated API. Since json events from loggly
can be arbitrary structures, this utility does not take responsibility to parse
and process this json. This responsibility is expected to be shared in
conjunction with a utility like jq which is specialized for parsing newline
delimited json events.


## Usage

### Help
```shell
$ loggly -h
loggly 0.1.0
Ayush Goyal <perfectayush@gmail.com>
Search logs from loggly cloud

USAGE:
    loggly [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Increase message verbosity

OPTIONS:
    -a, --account <ACCOUNT>    Loggly Account to search logs in use [env: LOGGLY_ACCOUNT=]
    -f, --from <FROM>          Time to start seaching from.
                               Can take absolute date in format 'yyyy-MM-dd HH:mm:ss.SSS'. Eg: `2020-08-13T18:45:00.000`
                               Or relative date values like `now`, last 10 mins `-10m`, last 5 weeks `-5w`, last 12
                               hours `-12h`
                               Or absolute Unix timestamp in *milliseconds* eg. `1617315836793` [default: -10m]
    -q, --query <Query>        Query search. Refer:
                               https://documentation.solarwinds.com/en/Success_Center/loggly/Content/admin/search-
                               query-language.htm  [default: *]
    -t, --token <API_TOKEN>    Loggly API token to use [env: LOGGLY_API_TOKEN=]
```

### Setup creds
You can pass loggly account and loggly api token info with flags above, or you can also pass this info in a environment variable.
```shell
export LOGGLY_ACCOUNT=<loggly_account_name>
export LOGGLY_API_TOKEN=<loggly_secret_api_token>
```

### Search
```shell
loggly -f -10s
```

### Use with jq
```shell
loggly -f -10s | jq
```

### Search with an ES like expression

Basic search can be done with ES like expressions. Refer loggly [Search query
language](https://documentation.solarwinds.com/en/Success_Center/loggly/Content/admin/search-query-language.htm)

```shell
loggly -f -12h -q 'json.hostname:stage01'
```

Query search with multiple search fields is also supported
```shell
loggly -f -12h -q 'json.hostname:stage01 json.status:400'
```


### Print specific fields with jq
```shell
loggly -f -12h -q 'json.hostname:stage01' | jq -c  '.event.json |"\(.hostname) \(.status)"'
```

# License
MIT License
