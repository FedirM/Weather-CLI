# Weather-CLI (test task)

It's a simple project is an answer for [Elastio Rust Test Task](https://gist.github.com/anelson/0029f620105a19702b5eed5935880a28#elastio-rust-test-task). Solution built based on [CLAP](https://crates.io/crates/clap) and [REQWEST](https://crates.io/crates/reqwest) libraries. All asynchronous code managed by [TOKYO](https://crates.io/crates/tokio) library. 
The CLI works with such APIs:
 - [Aeris Weather](https://www.aerisweather.com/)
 - [Open Weather](https://openweathermap.org/)


## Usage

Before all, run tests to check if everything works fine. To do that, build the project and run test command:

```
$>cargo build && cargo test
```


1. Setup configuration. You should specify API you'd prefer (optionally) and set ZIP-Code or City name the weather you are looking for:
```cmd
$>weather configure aeris --zip 51905
```

To chek your current configuration run:
```cmd
$>weather print
╭───────────┬───────────────╮
│ Parameter ┆ Value         │
╞═══════════╪═══════════════╡
│  API Name ┆ Aeris Weather │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│    City   ┆ N/A           │
├╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│  ZIP Code ┆ 51905         │
╰───────────┴───────────────╯
```

2. Now you could run the CLI to fetch the data. Also you could repick API from where the data would fetch, as in example below:

``` cmd
weather run -a open
╭─────────────────┬─────────────────────────────╮
│ Parameter       ┆ Value                       │
╞═════════════════╪═════════════════════════════╡
│      Place      ┆ Santiago Tianguistenco, USA │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│     Weather     ┆ broken clouds               │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│    Wind speed   ┆ 2.66 km/h                   │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│   Wind degree   ┆ 220°                        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│   Temperature   ┆ 24.91 C°                    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Temp feels like ┆ 24.55 C°                    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│     Pressure    ┆ 1010 hPA                    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│     Humidity    ┆ 42%                         │
╰─────────────────┴─────────────────────────────╯

```




