# Discord Bot in Rust using Poise

A simple Discord bot built in Rust using the Poise framework.

## Features

- **Color Roles**: Easy way to create colored roles.
- **Birthdays**: Reminds everyone about people's birthdays.

## Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [SQLite](https://www.sqlite.org/download.html)
- [Diesel CLI](https://diesel.rs/guides/getting-started/)

## Setup

1. Clone the repository:

    ```sh
    git clone https://github.com/ARKTEEK/purrislav.git
    cd purrislav
    ```

2. Set up the database:

    ```sh
    diesel setup
    diesel migration run
    ```

3. Create a `.env` file with your Discord token and database URL:

    ```env
    DISCORD_TOKEN=your-discord-bot-token-here
    DATABASE_URL=sqlite://db.sqlite
    ```

## Running the Bot

Start the bot with:

```sh
cargo run
