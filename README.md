# cowin-docker

Async Telegram bot written in Rust to monitor CoWin API for available vaccination slots.

A dockerized version of the rust vnb app (https://github.com/obviyus/vnb)

## Highlights
- **Exponential backoff** to gradually find a stable API query rate 
- [`teloxide`](https://github.com/teloxide/teloxide) framework using [`async`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
- **Strongly typed commands** and JSON decoding

## Setting up your environment
 1. Clone this repository and run:
 ```bash
$ git clone git@github.com:deveshmanish/vnb
$ cd vnb
```
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:abcdefgh`.
 3. Find your `CHANNEL_ID` in which alerts have to be sent using [@userinfobot](https://t.me/userinfobot)
 4. Edit the env_file and insert your bot token and channel id
```
TELOXIDE_TOKEN=<Your bot token here>
CHANNEL_ID=<Your channel_id here>
```
 5. **Adding or Removing new Districts**
- Get your state ID from: `https://cdn-api.co-vin.in/api/v2/admin/location/states`
- Get your district ID from: `https://cdn-api.co-vin.in/api/v2/admin/location/districts/<STATE_ID>`
- Open [`src/response.rs`](https://github.com/obviyus/vnb/blob/10cea6a460f52818730a1297c06239acd13dc692/src/response.rs#L45) in any text editor and change the `MONITORED_DISTRICTS` constant to your liking.:
5. Build and deploy the app in just one command:
```bash
$ docker-compose up --build -d
```

## TODO
- Streamline deployment by exporting monitored district constant so the image doesn't have to rebuild everytime.
- Logging and log rotation to be implemented.

## Contributing
Pull requests welcome!
