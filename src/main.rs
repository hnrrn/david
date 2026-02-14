use clap::{Parser, Subcommand};
use reqwest::Method;
use std::error::Error;

#[derive(Parser)]
#[command(name = "david")]
#[command(about = "A WebDAV client and server tool")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "WebDAV client commands")]
    Client {
        #[command(subcommand)]
        client_cmd: ClientCommands,

        #[arg(long, help = "WebDAV server URL")]
        url: String,

        #[arg(long, help = "Username")]
        user: Option<String>,

        #[arg(long, help = "Password")]
        pass: Option<String>,
    },
    #[command(about = "Start WebDAV server")]
    Server {
        #[arg(short, long, default_value = "8080", help = "Port to listen on")]
        port: u16,

        #[arg(short, long, default_value = ".", help = "Root directory to serve")]
        root: String,

        #[arg(long, help = "Username for server auth")]
        user: Option<String>,

        #[arg(long, help = "Password for server auth")]
        pass: Option<String>,
    },
}

#[derive(Subcommand)]
enum ClientCommands {
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

    match args.command {
        Commands::Client { client_cmd, url, user, pass } => {
            let client = reqwest::Client::new();
            let base_url = url.trim_end_matches('/');

            let auth = if let (Some(user), Some(pass)) = (user, pass) {
                Some((user, pass))
            } else {
                None
            };

            match client_cmd {
                ClientCommands::List { path } => {
                    let url = format!("{}{}", base_url, path);
                    let mut req = client.request(Method::from_bytes(b"PROPFIND").unwrap(), &url).header("Depth", "1");
                    if let Some((u, p)) = &auth {
                        req = req.basic_auth(u, Some(p));
                    }
                    let resp = req.send().await?;
                    let text = resp.text().await?;
                    println!("{}", text);
                }
                ClientCommands::Get { remote, local } => {
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
                ClientCommands::Put { local, remote } => {
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
                ClientCommands::Mkdir { path } => {
                    let url = format!("{}{}", base_url, path);
                    let mut req = client.request(Method::from_bytes(b"MKCOL").unwrap(), &url);
                    if let Some((u, p)) = &auth {
                        req = req.basic_auth(u, Some(p));
                    }
                    let resp = req.send().await?;
                    resp.error_for_status()?;
                    println!("Created directory {}", path);
                }
                ClientCommands::Delete { path } => {
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
        }
        Commands::Server { port, root, user, pass } => {
            println!("Starting WebDAV server on http://{}:{}", "0.0.0.0", port);
            if user.is_some() && pass.is_some() {
                println!("Basic auth enabled for user: {}", user.as_ref().unwrap());
            } else {
                println!("No auth configured");
            }
            // TODO: Implement WebDAV server with dav-server crate
            // For now, server mode is configured but not fully implemented
            println!("Server mode configured. Full implementation pending.");
        }
    }

    Ok(())
}