# This week in rust based blog posts

The idea behind this app is very simple. [This week in rust](https://github.com/emberian/this-week-in-rust) offers email subscription to recent updates in Rust community and also provides an interface to browse these updates online. Being a comprehensive knowledge base it still is very hard to browse for topics of interest since there is no way to find any particular blog post without knowing in which weekly update it is. This app aims to solve this problem by providing a simple blog post search mechanism and also by adding more Rust related blog post sources in the future.

## Installation

- Run the following command:
```shell script
cp .env.sample .env
```
- Setup `GITHUB_CLIENT_KEY` and `GITHUB_CLIENT_SECRET` in created `.env` file. This variables should use your actual github account name and account password since only basic auth scheme is supported.

## Running

Backend:
```shell script
cargo run --bin server
```

First run of backend might take some time since data would be fetched. You should expect to see log message as follows which would mean you are all set.

```
INFO - Starting "actix-web-service-127.0.0.1:8088" service on 127.0.0.1:8088
```

Prod version of server then becomes available at `http://127.0.0.1:8088`

Standalone Dev Frontend:
```shell script
yarn start
```

Dev front end then becomes available at `http://127.0.0.1:3000`

## Rebuilding prod front end

The following command is needed to rebuild the front-end app for prod. Note that when prod backend is run front-end is not recompiled so you would have to run this to make sure that you have the most up to date front-end version.
```shell script
cargo run --bin force-rebuild
```