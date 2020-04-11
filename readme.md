# This week in rust based blog posts

This is just a simple app which allows you to search through all the news titles and blog posts across all [this week in rust](https://github.com/emberian/this-week-in-rust) newsletters.

## Installation

- Run the following commands:
```shell script
cp .env.sample .env
yarn
```
- Setup `GITHUB_CLIENT_KEY` and `GITHUB_CLIENT_SECRET` in created `.env` file. This variables should use your actual github account name and account password since only basic auth scheme is supported.

## Running

Backend:
```shell script
cargo run
```

First run of backend might take some time since data would be fetched. You should expect to see log message as follows which would mean you are all set.

```
INFO - Starting "actix-web-service-127.0.0.1:8088" service on 127.0.0.1:8088
```

Frontend:
```shell script
yarn start
```

visit `http://localhost:3000/`