use regex::Regex;

// we want the messages that look like this:
// ... [10:18:46] [Server thread/INFO]: ....
// and we want to grab everything after the colon `:`
pub fn parse_message(msg: String) -> Option<String> {
    let re = Regex::new(r"minecraft-server.*Server thread.INFO.: (.*)").unwrap();
    let captures = re.captures(&msg)?;
    let message = captures[1].to_string();
    let message = filter_ip_from_login(message);
    let message = filter_ip(message);
    let message = filter_done(message);
    filter_noise(&message)?;
    return Some(message);
}

fn filter_noise(msg: &str) -> Option<&str> {
    let avoid = vec![
        r"^Loading properties.*",
        r"^Default game type.*",
        r"^Generating keypair.*",
        r"^Starting Minecraft server on.*",
        r"^Using epoll channel type.*",
        r"^Preparing level.*",
        r"^Preparing start region.*",
        r"^Time elapsed:.*",
    ];
    let res: Vec<bool> = avoid
        .iter()
        .map(|r| Regex::new(r).unwrap().is_match(&msg))
        .filter(|x| *x == true)
        .collect();
    match res.len() {
        0 => Some(msg),
        _ => None,
    }
}

// remove IP address from these messages:
// user[/111.111.111.111:12411] logged in with entity
fn filter_ip_from_login(msg: String) -> String {
    let re = Regex::new(r"(.+)\[\/[\d.:]+\] logged in with entity id .+ at (.*)").unwrap();
    let captures = re.captures(&msg);
    match captures.is_none() {
        true => msg,
        false => format!(
            "{} logged in at {}",
            captures.as_ref().unwrap()[1].to_string(),
            captures.unwrap()[2].to_string()
        ),
    }
}

fn filter_ip(msg: String) -> String {
    let re = Regex::new(r"(.+)\/\d+\.\d+\.\d+\.\d+:\d+(.*)").unwrap();
    let captures = re.captures(&msg);
    match captures.is_none() {
        true => msg,
        false => format!(
            "{} {}",
            captures.as_ref().unwrap()[1].to_string(),
            captures.unwrap()[2].to_string()
        ),
    }
}

fn filter_done(msg: String) -> String {
    let re = Regex::new(r"(Done \(.+\)!) For help, type .help.").unwrap();
    let captures = re.captures(&msg);
    match captures.is_none() {
        true => msg,
        false => format!("{}", captures.unwrap()[1].to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    #[test]
    fn filter_noise_test() {
        let msg = "Preparing start region for dimension minecraft:overworld\n";
        assert_eq!(None, filter_noise(msg));
        let msg = "Starting Minecraft server on *:25565";
        assert_eq!(None, filter_noise(msg));
        let msg = "Something else";
        assert_eq!(msg, filter_noise(msg).unwrap());
    }
    #[test]
    fn filter_done_test() {
        let msg = "Done (17.822s)! For help, type \"help\"";
        let res = filter_done(msg.to_string());
        assert_eq!(res, "Done (17.822s)!");
    }
    #[test]
    fn filter_ip_test() {
        let msg = "user[/111.111.111.111:12411] logged in with entity id 236 at 7128783078999";
        let res = filter_ip_from_login(msg.to_string());
        assert_eq!(res, "user logged in at 7128783078999");
        let msg_no_ip = "Done (17.822s)! For help, type help";
        let res = filter_ip_from_login(msg_no_ip.to_string());
        assert_eq!(res, msg_no_ip);
    }
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
}
