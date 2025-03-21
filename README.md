# Xeet

A command-line tool that lets you post to X (formerly Twitter) directly from your terminal.

## Installation

```bash
cargo install --path .
```

## Setup

1. Create a Twitter Developer account at [developer.twitter.com](https://developer.twitter.com/)

2. Create a new Project and App in the Developer Portal:

   - Go to the [Developer Portal](https://developer.twitter.com/en/portal/dashboard)
   - Click "Create Project"
   - Give your project a name and select "Production" as the environment
   - Select "Create App"

3. Enable read and write permissions:

   - In your app's settings, go to "User authentication settings"
   - Enable "OAuth 1.0a"
   - In the App Permissions section, select "Read and Write"
   - Save your changes

4. Get your API credentials:

   - Go to "Keys and Tokens" tab
   - Copy your "API Key" and "API Key Secret"
   - Generate "Access Token and Secret" with read and write permissions
   - Copy both the Access Token and Access Token Secret

5. Create a `.env` file in your project directory:
   ```
   X_CONSUMER_KEY=your_api_key
   X_CONSUMER_SECRET=your_api_secret
   X_ACCESS_TOKEN=your_access_token
   X_ACCESS_SECRET=your_access_token_secret
   ```

## Usage

To post a message:

```bash
xeet post -- "Your message here"
```

To view setup instructions:

```bash
xeet setup
```

## Features

- Post to X directly from your terminal
- OAuth 1.0a authentication
- Colored terminal output
- Helpful setup instructions

## License

MIT
