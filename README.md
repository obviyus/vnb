# vnb [![CI](https://github.com/obviyus/vnb/actions/workflows/main.yml/badge.svg)](https://github.com/obviyus/vnb/actions/workflows/main.yml) ![Lines of code](https://img.shields.io/tokei/lines/github/obviyus/vnb)
Async Telegram bot written in Rust to monitor CoWin API for available vaccination slots.

You can find one instance of this bot running at [@COVID_Vaccine_Updates](https://t.me/COVID_Vaccine_Updates)

## Highlights
- **Exponential backoff** to gradually find a stable API query rate 
- [`teloxide`](https://github.com/teloxide/teloxide) framework using [`async`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
- **Strongly typed commands** and JSON decoding

## Setting up your environment
 1. [Download Rust](http://rustup.rs/).
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:abcdefgh`.
 3. Initialise the `TELOXIDE_TOKEN`, `CHANNEL_ID` and `OWNER_ID` (optional) environmental variables to your tokens and IDs. You can find your `CHANNEL_ID` and `OWNER_ID` using [@userinfobot](https://t.me/userinfobot) 
```bash
$ export TELOXIDE_TOKEN=<Your token here>
$ export CHANNEL_ID=<Your channel_id here>
$ export OWNER_ID=<Your owner_id here>
...
```
 4. Make sure that your Rust compiler is up to date:
```bash
$ rustup update stable
$ rustup override set stable
```
5. Clone this repository and run:
```bash
$ git clone git@github.com:obviyus/vnb
$ cd vnb
$ cargo run
```

## Adding or Removing new Districts
- Get your state ID from: `https://cdn-api.co-vin.in/api/v2/admin/location/states`
- Get your district ID from: `https://cdn-api.co-vin.in/api/v2/admin/location/districts/<STATE_ID>`
- Open [`src/response.rs`](https://github.com/obviyus/vnb/blob/10cea6a460f52818730a1297c06239acd13dc692/src/response.rs#L45) in any text editor and change the `MONITORED_DISTRICTS` constant to your liking.

## TODO
- Streamline deployment using Docker
- Reduce binary size (current: **9.8MB**)

## Contributing
Pull requests welcome!
