# SUN

Sun is a simple-to-use command line weather app written in Rust.

## Setup

### Prerequisites

Setup an environment variable named "WEATHER_API_KEY" containing your secret api key or use the .env file.

```sh
WEATHER_API_KEY="your-unique-key"
```
 
## How to build

To build the project use the following command :

```sh
cargo build --release
```

To build and run the project use the following command:

```sh
cargo run --release
```

## How to install

You can also install the application using the following command :

```sh
cargo install --path . --locked
```

## How to use

```sh
sun <location>
```