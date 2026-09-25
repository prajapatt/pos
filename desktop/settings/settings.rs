#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Setting { pub key: String, pub value: String }
#[derive(Default)]
pub struct SettingsStore { values: Vec<Setting> }
impl SettingsStore { pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) { let key = key.into(); if let Some(setting) = self.values.iter_mut().find(|setting| setting.key == key) { setting.value = value.into(); } else { self.values.push(Setting { key, value: value.into() }); } } pub fn get(&self, key: &str) -> Option<&str> { self.values.iter().find(|setting| setting.key == key).map(|setting| setting.value.as_str()) } }
