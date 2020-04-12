# This week in rust based blog posts

The idea behind this app is very simple. [This week in rust](https://github.com/emberian/this-week-in-rust) offers email subscription to recent updates in Rust community and also provides an interface to browse these updates online. Being a comprehensive knowledge base it still is very hard to browse for topics of interest since there is no way to find any particular blog post without knowing in which weekly update it is. This app aims to solve this problem by providing a simple blog post search mechanism and also by adding more Rust related blog post sources in the future.

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