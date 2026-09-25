#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DnsRecord {
    pub host: String,
    pub address: String,
}

#[derive(Clone, Debug, Default)]
pub struct DnsResolver {
    cache: Vec<DnsRecord>,
}

impl DnsResolver {
    pub fn lookup(&self, host: &str) -> Option<DnsRecord> {
        self.cache.iter().find(|record| record.host == host).cloned()
    }

    pub fn cache_record(&mut self, host: &str, address: &str) {
        self.cache.push(DnsRecord {
            host: host.to_string(),
            address: address.to_string(),
        });
    }
}
