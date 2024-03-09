use log::{error, info};
use std::env;
use std::process::{Command, Stdio};
use std::{process, thread, time};

use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use serenity::model::webhook::Webhook;

mod logger;

// We will use the cmd `netstat -atuen | grep 25565.*ESTABLISHED` to determine if there are active
// connections to the minecraft server, this means there are players currently online.
fn netstat_established() -> bool {
    let ls = Command::new("netstat") // netstat -atuen |
        .arg("-atuen")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut grep = Command::new("grep")
        .arg(":25565.*ESTABLISHED")
        // looks for TCP connections on port 25565 that are ESTABLISHED, we know these are the
        // minecraft connections because we configured the server to use port 25565. A bit hacky to
        // parse netstat but it should work for now!
        .stdin(Stdio::from(ls.stdout.unwrap()))
        .spawn()
        .unwrap();
    let ecode = grep.wait().expect("failed to wait on grep");
    return ecode.success();
    // `true` means that grep found a match, thefore there is an active connection
    // `false` means that grep did not find a match, therefore there is no active MC connection
}

fn can_sudo() -> bool {
    // This is fine during test if the user running the CMD has sudo privileges.
    // When running as a service, the sudo privileges must be given to the process running the CMD.
    let sudo_cmd = Command::new("sudo").arg("-v").status().unwrap();
    return sudo_cmd.success();
}

// To debug:
#[allow(dead_code)]
fn test_grep() -> bool {
    let ls = Command::new("ls") // netstat -atuen | grep :25565.*ESTABLISHED
        .arg("-la")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut grep = Command::new("grep")
        .arg("testfile")
        .stdin(Stdio::from(ls.stdout.unwrap()))
        .spawn()
        .unwrap();
    let ecode = grep.wait().expect("failed to wait on grep");
    match ecode.success() {
        true => return true,   // this means that grep found a match
        false => return false, // this means that grep did not find a match
    };
}

async fn shutdown_server() {
    send_message("Shutting down server.").await;
    info!("Shut down sequence started.");
    let shutdown = Command::new("sudo")
        .arg("shutdown")
        .arg("now")
        .arg("-h")
        .status()
        .unwrap();

    if shutdown.success() {
        info!("Shutting down...");
        process::exit(0);
    } else {
        error!("Failed to shut down!: {shutdown}.");
    }
}

async fn send_message(message: &str) {
    let webhook_url =
        env::var("DISCORD_WEBHOOK_URL").expect("Expected DISCORD_WEBHOOK_URL in the environment");
    let http = Http::new("");
    let webhook_url = webhook_url.to_string();
    let webhook = Webhook::from_url(&http, &webhook_url).await.unwrap();
    let builder = ExecuteWebhook::new().content(message);
    let res = webhook.execute(&http, false, builder).await;
    if let Err(why) = res {
        eprintln!("Error sending message: {why:?}");
    };
}

#[tokio::main]
async fn main() {
    info!("Minecraft Watcher has launched.");
    let _ = logger::init();
    let mut last_active = time::Instant::now();
    // To debug:
    // let wait_time = time::Duration::from_secs(4);
    let wait_time = time::Duration::from_secs(5 * 60);
    match can_sudo() {
        true => info!("Sudo privileges are valid."),
        false => error!("Minecraft watcher does not have sudo privileges!"),
    }
    loop {
        thread::sleep(wait_time);
        let now = time::Instant::now();
        let minutes_since_last_active = (now - last_active).as_secs() / 60;
        // To debug:
        // let minutes_since_last_active = (now - last_active).as_secs();
        match netstat_established() {
            true => {
                info!("Active connection found");
                last_active = time::Instant::now();
            }
            false => {
                info!("Zero connections found");
                if minutes_since_last_active > 15 {
                    shutdown_server().await;
                };
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    #[test]
    fn time_greater_than() {
        let past = time::Instant::now();
        let wait_time = time::Duration::from_millis(4);
        thread::sleep(wait_time);
        let present = time::Instant::now();
        let result = past < present;
        assert_eq!(result, true);
    }
    #[test]
    fn time_difference() {
        let past = time::Instant::now();
        let wait_time = time::Duration::from_millis(4);
        thread::sleep(wait_time);
        let present = time::Instant::now();
        let result = present - past;
        assert!(result.as_millis() >= 4);
    }
    #[ignore] // ignore it during build because "sudo" isn't found by nix
    #[test]
    fn sudo() {
        // This is fine during test if the user running the CMD has sudo privileges.
        // When running as a service, the sudo privileges must be given to the process running the CMD.
        assert!(can_sudo())
    }
}
