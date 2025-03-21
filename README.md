# Xeet

Terminal-based shitposting for chronically online devs who can't be bothered to open a browser. Post to X (formerly Twitter) without ever leaving your precious terminal. #TouchGrass

## Installation

```bash
cargo install --path .
```

## SPEED-RUN Setup (any%)

1. Get API keys from [developer.twitter.com](https://developer.twitter.com/)

   - Make a project/app, enable OAuth 1.0a
   - Set to "Read and Write" perms (IMPORTANT)
   - Grab API Key, API Secret, Access Token, and Access Token Secret

2. Create a global config file (works from ANY directory, no cap):

   ```bash
   # For MacOS/Linux enjoyers:
   mkdir -p ~/.config/xeet
   nano ~/.config/xeet/config.toml

   # For Windows NPCs:
   mkdir -p %APPDATA%\xeet
   notepad %APPDATA%\xeet\config.toml
   ```

3. Paste this in your config file and you're BASED:
   ```toml
   [credentials]
   consumer_key = "your_api_key"
   consumer_secret = "your_api_secret"
   access_token = "your_access_token"
   access_secret = "your_access_token_secret"
   ```

## Usage (aka How to Main Character)

Basic shitposting:

```bash
xeet post -- "Hot take: semicolons in JavaScript are just spicy line breaks"
```

Need setup help? Type:

```bash
xeet setup
```

## Pro Strats (Touch Grass Any% Speedrun)

### God-tier Aliases

Add to your `.bashrc` or `.zshrc`:

```bash
# For maximum efficiency bruh
alias x="xeet post --"
alias take="xeet post -- 'Hot take:'"
alias fr="xeet post -- 'fr fr no cap'"
alias npc="xeet post -- 'NPC behavior'"
```

Now you can post with:

```bash
x "This meeting could've been an email frfr"
take "IDEs are just fancy text editors"
fr
npc
```

### Keyboard Shortcuts

Set up function keys in your terminal config:

```bash
bind -x '"\e[21~": "xeet post -- \"skill issue\""'  # F10 for instant L posting
```

## Features

- Terminal shitposting (no browser = based)
- OAuth authentication (secure-pilled)
- Works from ANY directory (location-pilled)
- Won't get you maidenless (results may vary)

## License

MIT (it's giving generous)
