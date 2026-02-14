use clap::{Parser, Subcommand};
use reqwest::Method;
use std::error::Error;

#[derive(Parser)]
#[command(name = "david")]
#[command(about = "A WebDAV CLI tool")]
struct Args {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, help = "WebDAV server URL")]
    url: String,

    #[arg(long, help = "Username")]
    user: Option<String>,

    #[arg(long, help = "Password")]
    pass: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List directory contents (PROPFIND response)")]
    List {
        #[arg(default_value = "/")]
        path: String,
    },
    #[command(about = "Download file")]
    Get {
        #[arg(help = "Remote path")]
        remote: String,
        #[arg(help = "Local path")]
        local: String,
    },
    #[command(about = "Upload file")]
    Put {
        #[arg(help = "Local path")]
        local: String,
        #[arg(help = "Remote path")]
        remote: String,
    },
    #[command(about = "Create directory")]
    Mkdir {
        #[arg(help = "Path to create")]
        path: String,
    },
    #[command(about = "Delete file or directory")]
    Delete {
        #[arg(help = "Path to delete")]
        path: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = reqwest::Client::new();
    let base_url = args.url.trim_end_matches('/');

    let auth = if let (Some(user), Some(pass)) = (args.user, args.pass) {
        Some((user, pass))
    } else {
        None
    };

    match args.command {
        Commands::List { path } => {
            let url = format!("{}{}", base_url, path);
            let mut req = client.request(Method::PROPFIND, &url).header("Depth", "1");
            if let Some((u, p)) = &auth {
                req = req.basic_auth(u, Some(p));
            }
            let resp = req.send().await?;
            let text = resp.text().await?;
            println!("{}", text);
        }
        Commands::Get { remote, local } => {
            let url = format!("{}{}", base_url, remote);
            let mut req = client.get(&url);
            if let Some((u, p)) = &auth {
                req = req.basic_auth(u, Some(p));
            }
            let resp = req.send().await?;
            let bytes = resp.bytes().await?;
            std::fs::write(&local, bytes)?;
            println!("Downloaded {} to {}", remote, local);
        }
        Commands::Put { local, remote } => {
            let data = std::fs::read(&local)?;
            let url = format!("{}{}", base_url, remote);
            let mut req = client.put(&url);
            if let Some((u, p)) = &auth {
                req = req.basic_auth(u, Some(p));
            }
            let resp = req.body(data).send().await?;
            resp.error_for_status()?;
            println!("Uploaded {} to {}", local, remote);
        }
        Commands::Mkdir { path } => {
            let url = format!("{}{}", base_url, path);
            let mut req = client.request(Method::from_bytes(b"MKCOL").unwrap(), &url);
            if let Some((u, p)) = &auth {
                req = req.basic_auth(u, Some(p));
            }
            let resp = req.send().await?;
            resp.error_for_status()?;
            println!("Created directory {}", path);
        }
        Commands::Delete { path } => {
            let url = format!("{}{}", base_url, path);
            let mut req = client.delete(&url);
            if let Some((u, p)) = &auth {
                req = req.basic_auth(u, Some(p));
            }
            let resp = req.send().await?;
            resp.error_for_status()?;
            println!("Deleted {}", path);
        }
    }

    Ok(())
}