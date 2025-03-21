use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use toml;
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

#[derive(Parser)]
#[clap(name = "xeet", about = "Xeet from your terminal", version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a xeet
    Post {
        /// Number of minutes to wait before sending the xeet (optional)
        #[clap(short, long)]
        schedule: Option<u64>,

        /// The text of your xeet
        #[clap(last = true)]
        text: String,
    },
    /// Setup your Twitter credentials
    Setup,
}

struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_secret: String,
}

impl Config {
    fn from_env() -> Result<Self> {
        // Only use global config file
        Self::from_config_file()
    }

    fn from_config_file() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        let config_str = fs::read_to_string(&config_path).context(format!(
            "Failed to read config file at {:?}. Run 'xeet setup' to create it.",
            config_path
        ))?;

        let parsed_config: toml::Value =
            toml::from_str(&config_str).context("Failed to parse config file as TOML")?;

        let creds = parsed_config
            .get("credentials")
            .context("No 'credentials' section in config file")?;

        let consumer_key = creds
            .get("consumer_key")
            .and_then(|v| v.as_str())
            .context("Missing consumer_key in config")?
            .to_string();

        let consumer_secret = creds
            .get("consumer_secret")
            .and_then(|v| v.as_str())
            .context("Missing consumer_secret in config")?
            .to_string();

        let access_token = creds
            .get("access_token")
            .and_then(|v| v.as_str())
            .context("Missing access_token in config")?
            .to_string();

        let access_secret = creds
            .get("access_secret")
            .and_then(|v| v.as_str())
            .context("Missing access_secret in config")?
            .to_string();

        Ok(Config {
            consumer_key,
            consumer_secret,
            access_token,
            access_secret,
        })
    }

    fn get_config_path() -> Result<PathBuf> {
        let home = if cfg!(windows) {
            env::var("APPDATA").context("APPDATA environment variable not set")?
        } else {
            let home = env::var("HOME").context("HOME environment variable not set")?;
            format!("{}/.config", home)
        };

        let config_dir = if cfg!(windows) {
            format!("{}/xeet", home)
        } else {
            format!("{}/xeet", home)
        };

        Ok(PathBuf::from(format!("{}/config.toml", config_dir)))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Post { schedule, text } => match schedule {
            Some(minutes) => {
                schedule_xeet(minutes, text).await?;
            }
            None => {
                send_xeet(text).await?;
            }
        },
        Commands::Setup => {
            setup()?;
        }
    }

    Ok(())
}

async fn send_xeet(text: String) -> Result<()> {
    let config = Config::from_env()?;

    let auth = Oauth1aToken::new(
        config.consumer_key,
        config.consumer_secret,
        config.access_token,
        config.access_secret,
    );

    let twitter_client = TwitterApi::new(auth);

    match twitter_client.post_tweet().text(text).send().await {
        Ok(response) => {
            if let Some(ref xeet) = response.data {
                println!("{} {}", "✓".green().bold(), xeet.id);
            } else {
                println!("{}", "✓".green().bold());
            }
            Ok(())
        }
        Err(e) => {
            println!("{} {}", "✗".red().bold(), e);
            anyhow::bail!("Failed to send xeet")
        }
    }
}

fn setup() -> Result<()> {
    let config_path = Config::get_config_path()?;
    let config_dir = config_path.parent().unwrap();

    // Create config directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).context("Failed to create config directory")?;
    }

    println!("{}", "Setup:".cyan().bold());

    let config_location = if cfg!(windows) {
        format!("%APPDATA%\\xeet\\config.toml")
    } else {
        format!("~/.config/xeet/config.toml")
    };

    println!("1. Get API keys @ {}", "developer.x.com".green());
    println!("2. Create TOML @ {}", config_location.yellow());
    println!("");
    println!("[credentials]");
    println!("consumer_key = \"your_api_key\"");
    println!("consumer_secret = \"your_api_secret\"");
    println!("access_token = \"your_access_token\"");
    println!("access_secret = \"your_access_token_secret\"");

    Ok(())
}

async fn schedule_xeet(minutes: u64, text: String) -> Result<()> {
    println!("{} Scheduling post for {} minutes from now", "⏰", minutes);

    // Convert minutes to seconds
    let delay_secs = minutes * 60;

    // Create a command to schedule the post using another process
    let current_exe = env::current_exe().context("Failed to get current executable path")?;

    // Build the command that will be run in the background
    let mut command = if cfg!(windows) {
        let mut cmd = std::process::Command::new("cmd");
        cmd.args([
            "/C",
            &format!(
                "timeout /T {} /NOBREAK > nul && \"{}\" post -- \"{}\"",
                delay_secs,
                current_exe.display(),
                text.replace("\"", "\\\"")
            ),
        ]);
        cmd
    } else {
        let mut cmd = std::process::Command::new("sh");
        cmd.args([
            "-c",
            &format!(
                "sleep {} && \"{}\" post -- \"{}\" &",
                delay_secs,
                current_exe.display(),
                text.replace("\"", "\\\"")
            ),
        ]);
        cmd
    };

    // Detach the process so it runs in the background
    if !cfg!(windows) {
        command
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
    }

    // Spawn the process
    match command.spawn() {
        Ok(_) => {
            println!(
                "{} Post scheduled successfully for {}. Terminal is free to use.",
                "✓".green().bold(),
                chrono::Local::now()
                    .checked_add_signed(chrono::Duration::minutes(minutes as i64))
                    .map(|dt| dt.format("%H:%M:%S").to_string())
                    .unwrap_or_else(|| format!("{} minutes from now", minutes))
            );
        }
        Err(e) => {
            println!("{} Failed to schedule post: {}", "✗".red().bold(), e);
            anyhow::bail!("Failed to schedule post");
        }
    }

    Ok(())
}
