# Xeet

A command-line tool that lets you tweet directly from your terminal.

## Installation

```bash
cargo install --path .
```

## Setup

1. Create a Twitter Developer account at [developer.twitter.com](https://developer.twitter.com/)
2. Create a Twitter app and get your API credentials
3. Copy `.env.example` to `.env` and fill in your Twitter API credentials

## Usage

To send a tweet:

```bash
xeet tweet "Hello, world from my terminal!"
```

To view your timeline:

```bash
xeet timeline
```

## Features

- Tweet directly from your terminal
- View your Twitter timeline
- Simple and easy to use

## License

MIT
