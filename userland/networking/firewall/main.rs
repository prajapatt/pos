use std::env;
use std::process::ExitCode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirewallRule {
    pub direction: String,
    pub protocol: String,
    pub port: u16,
    pub action: String,
}

impl FirewallRule {
    fn new(direction: impl Into<String>, protocol: impl Into<String>, port: u16, action: impl Into<String>) -> Self {
        Self {
            direction: direction.into(),
            protocol: protocol.into(),
            port,
            action: action.into(),
        }
    }
}

fn list_rules(rules: &[FirewallRule]) {
    for rule in rules {
        println!("{} {} {} {}", rule.direction, rule.protocol, rule.port, rule.action);
    }
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let mut rules: Vec<FirewallRule> = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "allow" => {
                let direction = args.next().unwrap_or_else(|| "in".to_string());
                let protocol = args.next().unwrap_or_else(|| "tcp".to_string());
                let port = args.next().unwrap_or_else(|| "80".to_string()).parse::<u16>().unwrap_or(80);
                rules.push(FirewallRule::new(direction, protocol, port, "allow"));
            }
            "deny" => {
                let direction = args.next().unwrap_or_else(|| "in".to_string());
                let protocol = args.next().unwrap_or_else(|| "tcp".to_string());
                let port = args.next().unwrap_or_else(|| "443".to_string()).parse::<u16>().unwrap_or(443);
                rules.push(FirewallRule::new(direction, protocol, port, "deny"));
            }
            "list" => {
                list_rules(&rules);
                return ExitCode::SUCCESS;
            }
            "clear" => {
                rules.clear();
                println!("firewall rules cleared");
                return ExitCode::SUCCESS;
            }
            _ => {
                eprintln!("usage: firewall [allow|deny|list|clear]");
                return ExitCode::from(2);
            }
        }
    }

    if rules.is_empty() {
        println!("firewall: no active rules");
        return ExitCode::SUCCESS;
    }

    list_rules(&rules);
    ExitCode::SUCCESS
}
