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

Schedule a post for later (touch grass in the meantime):

```bash
xeet post --schedule 30 -- "Just had a great idea while touching grass, semicolons in JavaScript are just spicy line breaks"
```

Or use the shorthand version:

```bash
xeet post -s 30 -- "I'm totally working and did not go out for a walk 30 minutes ago"
```

Scheduled posts run in the background, so your terminal remains free to use for other tasks!

If a scheduled post fails, you can check the logs:

```bash
xeet logs      # Shows the latest log
xeet logs -a   # Shows all logs
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
alias xs="xeet post -s"  # Using shorthand for schedule
alias fr="xeet post -- 'fr fr no cap'"
alias npc="xeet post -- 'yesss true'"
alias npcrypto="xeet post -- 'gm!'"
```

Now you can post with:

```bash
x "This meeting could've been an email frfr"
xs 30 -- "Scheduled post 30 minutes from now"
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
- Schedule posts for later (runs in background, keep using your terminal)
- Won't get you maidenless (results may vary)

## License

MIT (it's giving generous)
