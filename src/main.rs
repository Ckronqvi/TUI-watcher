use clap::{Parser, Subcommand};
use dotenv::dotenv;
use tui_watcher::{get_tui_price, send_email};
use std::error::Error;
use daemonize::Daemonize;
use std::fs::{File, read_to_string};
use std::time::Duration;
use tokio::time::interval;
use std::process::{exit, Command};
use std::path::Path;

mod io;

#[derive(Parser)]
#[command(name = "TUI Watcher")]
#[command(about = 
    "A CLI tool to watch TUI prices.
    -- get-price <url> retrieves the price. -- watch <url> <email> 
    starts a watcher and emails you if the price changes. Use -- daemon to daemonize the watcher",
    long_about = None
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    GetPrice { url: String },
    Watch {
      url: String,
      email: String,
      #[arg(short, long)]
      daemon: bool
  },
  Stop,
}

fn main() -> Result<(), Box<dyn Error>> {
  // load env variables before potential daemonizing
  dotenv().ok();
  let cli = Cli::parse();
  match &cli.command {
   Commands::Watch {url: _, email: _, daemon} => {
      if *daemon {
        let stdout = File::create("/tmp/tui-watcher.out")?;
        let stderr = File::create("/tmp/tui-watcher.err")?;
        let pid_file = "/tmp/tui-watcher.pid";
        let daemonize = Daemonize::new()
          .stdout(stdout)
          .stderr(stderr)
          .pid_file(pid_file)
          .chown_pid_file(true);
        
        match daemonize.start() {
            Ok(_) => println!(
                              "Started the watcher successfully, PID file: {}. To kill: kill $(cat /tmp/tui-watcher.pid)
                              or by running the -- stop command", pid_file
                     ),
            Err(e) => eprintln!("Error daemonizing: {}", e),
        }
      }
    }
    _ => {}
  }
  tokio_main()
}

#[tokio::main]
async fn tokio_main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::parse();
  match &cli.command {
    Commands::GetPrice {url} => {
      let price = get_tui_price(&url).await?; 
      println!("Current price is: {}", price);
    },
    Commands::Watch { url, email, daemon: _ } => {
      watch_tui_price(url, email).await?;
    } 
    Commands::Stop => {
      stop_daemon()?;
    }
  }
  Ok(())
}

async fn watch_tui_price(url: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let mut interval = interval(Duration::from_secs(60*120)); // 2 hours
    let mut last_price = match io::read_last_price() {
      Some(v) => v,
      None => {
        let price = get_tui_price(url).await?;
        io::save_last_price(&price)?;
        price
      }, 
    };
    
    loop {
        interval.tick().await;
        let current_price = get_tui_price(url).await?;
        if current_price != last_price {
          let result = send_email(&current_price, &last_price, email).is_ok();
          if !result {
            println!("Error while sending email. Ending...");
            stop_daemon().unwrap();
            exit(1);
          }
          io::save_last_price(&current_price)?;
          last_price = current_price.clone();
        }
    }
}

fn stop_daemon() -> Result<(), Box<dyn Error>> {
    let pid_file = "/tmp/tui-watcher.pid";
    if Path::new(pid_file).exists() {
        let pid = read_to_string(pid_file)?.trim().parse::<i32>()?;
        Command::new("kill").arg(pid.to_string()).output()?;
        println!("Daemon stopped successfully.");
    } else {
        println!("No PID file found. Is the daemon running?");
    }
    Ok(())
}
