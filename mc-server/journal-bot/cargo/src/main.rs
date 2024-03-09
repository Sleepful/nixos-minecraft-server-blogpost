use regex::Regex;
use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use serenity::builder::ExecuteWebhook;
use serenity::http::Http;
use serenity::model::webhook::Webhook;

struct ClientData {
    webhook_url: String,
    http: Http,
}

// we want the messages that look like this:
// ... [10:18:46] [Server thread/INFO]: ....
// and we want to grab everything after the colon `:`
fn parse_message(msg: String) -> Option<String> {
    let re = Regex::new(r"minecraft-server.*Server thread.INFO.: (.*)").unwrap();
    let captures = re.captures(&msg);
    match captures.is_none() {
        true => None,
        _ => Some(captures.unwrap()[1].to_string()),
    }
}

#[allow(dead_code)]
async fn listen_to_journalctl(cd: ClientData) {
    // journalctl --follow -u minecraft-server -b
    let cmd = Command::new("journalctl")
        .arg("--follow")
        .arg("-u")
        .arg("minecraft-server")
        .arg("-b") // this gives only the logs since the system last booted
        .stdout(Stdio::piped())
        .spawn()
        .expect("journalctl CMD Failed");
    let mut out = cmd.stdout.unwrap();
    let mut reader = BufReader::new(&mut out);
    loop {
        let mut line = String::new();
        let num_bytes = reader.read_line(&mut line).unwrap();
        if num_bytes == 0 {
            println!("EOF"); // we should not get here with journalctl
            break;
        }
        if let Some(valid_msg) = parse_message(line) {
            send_message(&cd, &valid_msg).await;
        }
    }
    println!("Done."); // we should not get here with journalctl
}

#[allow(dead_code)]
async fn listen_to_slow_output(cd: ClientData) {
    let cmd = Command::new("./test_cmd/target/debug/slow_output")
        .stdout(Stdio::piped())
        .spawn()
        .expect("CMD Failed");
    let mut out = cmd.stdout.unwrap();
    let mut reader = BufReader::new(&mut out);
    loop {
        let mut line = String::new();
        let num_bytes = reader.read_line(&mut line).unwrap();
        if num_bytes == 0 {
            println!("EOF");
            break;
        }
        send_message(&cd, &line).await;
    }
    println!("Done.");
}

async fn send_message(cd: &ClientData, message: &str) {
    let webhook = Webhook::from_url(&cd.http, &cd.webhook_url).await.unwrap();
    let builder = ExecuteWebhook::new().content(message);
    let res = webhook.execute(&cd.http, false, builder).await;
    if let Err(why) = res {
        eprintln!("Error sending message: {why:?}");
    };
}

#[tokio::main]
async fn main() {
    let webhook_url =
        env::var("DISCORD_WEBHOOK_URL").expect("Expected DISCORD_WEBHOOK_URL in the environment");

    let client_data = ClientData {
        http: Http::new(""),
        webhook_url: webhook_url.to_string(),
    };

    listen_to_journalctl(client_data).await;
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    #[test]
    // Mar 08 10:18:54 MC-Server minecraft-server[970]: [10:18:54] [Worker-Main-1/INFO]: Preparing spawn area: 7%
    fn regex_match() {
        let re = Regex::new(r"minecraft-server.*Server thread.INFO.: (.*)").unwrap();
        let hay = "Mar 08 10:18:54 MC-Server minecraft-server[970]: [10:18:54] [Server thread/INFO]: Done (16.845s)! For help, type help";
        let caps = re.captures(hay).unwrap();
        assert_eq!("Done (16.845s)! For help, type help", &caps[1]);
    }
    #[test]
    fn regex_no_match() {
        let no_hay = "Mar 08 10:18:54 MC-Server minecraft-server[970]: [10:18:54] [Worker-Main-1/INFO]: Preparing spawn area: 7%";
        let re = Regex::new(r"minecraft-server.*Server thread.INFO.: (.*)").unwrap();
        let no_caps = re.captures(no_hay);
        assert!(no_caps.is_none());
    }
    // $ cargo test -- --nocapture
    // for this test, compile the ./test_cmd app with `cargo build`
    use std::io::{BufRead, BufReader};
    use std::process::{Command, Stdio};
    #[ignore]
    #[test]
    fn test_stdout() {
        let cmd = Command::new("./test_cmd/target/debug/slow_output")
            .stdout(Stdio::piped())
            .spawn()
            .expect("CMD Failed");
        let mut out = cmd.stdout.unwrap();

        let mut reader = BufReader::new(&mut out);
        loop {
            let mut line = String::new();
            let num_bytes = reader.read_line(&mut line).unwrap();
            if num_bytes == 0 {
                println!("EOF");
                break;
            }
            println!("The line: {:?}", line);
        }
        println!("Done.");
    }
}
