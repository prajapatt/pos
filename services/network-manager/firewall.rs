#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
    Reject,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirewallRule {
    pub source: String,
    pub destination: String,
    pub action: FirewallAction,
}

#[derive(Clone, Debug, Default)]
pub struct FirewallPolicy {
    rules: Vec<FirewallRule>,
}

impl FirewallPolicy {
    pub fn insert(&mut self, rule: FirewallRule) {
        self.rules.push(rule);
    }

    pub fn allows(&self, source: &str, destination: &str) -> bool {
        self.rules.iter().all(|rule| {
            !(rule.source == source && rule.destination == destination && rule.action == FirewallAction::Deny)
        })
    }
}
