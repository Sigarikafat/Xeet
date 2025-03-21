use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use dotenv::dotenv;
use std::env;
use twitter_v2::{
    authorization::Oauth1aToken,
    TwitterApi,
};

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
        dotenv().ok();
        
        let consumer_key = env::var("X_CONSUMER_KEY")
            .context("X_CONSUMER_KEY not found in environment")?;
        let consumer_secret = env::var("X_CONSUMER_SECRET")
            .context("X_CONSUMER_SECRET not found in environment")?;
        let access_token = env::var("X_ACCESS_TOKEN")
            .context("X_ACCESS_TOKEN not found in environment")?;
        let access_secret = env::var("X_ACCESS_SECRET")
            .context("X_ACCESS_SECRET not found in environment")?;
        
        Ok(Config {
            consumer_key,
            consumer_secret,
            access_token,
            access_secret,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Post { text } => {
            send_xeet(text).await?;
        }
        Commands::Setup => {
            setup()?;
        }
    }

    Ok(())
}

async fn send_xeet(text: String) -> Result<()> {
    println!("{}", "Sending xeet...".cyan());
    
    let config = Config::from_env()?;
    
    // Create an OAuth 1.0a token with the Twitter credentials
    let auth = Oauth1aToken::new(
        config.consumer_key,
        config.consumer_secret,
        config.access_token,
        config.access_secret,
    );
    
    // Create a Twitter API client with the auth token
    let twitter_client = TwitterApi::new(auth);
    
    // Send the xeet
    match twitter_client.post_tweet().text(text.clone()).send().await {
        Ok(response) => {
            println!("{}", "Xeet sent successfully:".green().bold());
            println!("{}", text);
            if let Some(ref xeet) = response.data {
                println!("Xeet ID: {}", xeet.id);
            }
            Ok(())
        },
        Err(e) => {
            println!("{}", "Failed to send xeet:".red().bold());
            println!("{}", e);
            anyhow::bail!("Failed to send xeet: {}", e)
        }
    }
}

fn setup() -> Result<()> {
    println!("{}", "Setting up Twitter credentials...".cyan());
    println!("To use Xeet, you need to:");
    println!("1. Create a Twitter Developer account at {}", "https://developer.x.com/en/portal/dashboard".green());
    println!("2. Create a new Project and App in the Developer Portal:");
    println!("3. Enable read and write permissions:");
    println!("4. Fill in the .env file with your credentials, see .env.example for reference:");
    println!("   X_CONSUMER_KEY=your_api_key");
    println!("   X_CONSUMER_SECRET=your_api_secret");
    println!("   X_ACCESS_TOKEN=your_access_token");
    println!("   X_ACCESS_SECRET=your_access_token_secret");
    
    Ok(())
}
