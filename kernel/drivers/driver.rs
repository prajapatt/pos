#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DriverState {
    New,
    Probed,
    Online,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DriverRecord {
    pub name: &'static str,
    pub state: DriverState,
}

impl DriverRecord {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            state: DriverState::New,
        }
    }
}

pub trait Driver {
    fn name(&self) -> &'static str;
    fn probe(&mut self) -> Result<(), &'static str>;
    fn state(&self) -> DriverState;
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DriverRegistry {
    drivers: Vec<DriverRecord>,
}

impl DriverRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: &'static str) -> bool {
        if self.drivers.iter().any(|driver| driver.name == name) {
            return false;
        }

        self.drivers.push(DriverRecord::new(name));
        true
    }

    pub fn state(&self, name: &'static str) -> Option<DriverState> {
        self.drivers
            .iter()
            .find(|driver| driver.name == name)
            .map(|driver| driver.state)
    }

    pub fn probe(&mut self, name: &'static str) -> Result<(), &'static str> {
        let driver = self
            .drivers
            .iter_mut()
            .find(|driver| driver.name == name)
            .ok_or("driver not found")?;

        driver.state = DriverState::Probed;
        Ok(())
    }

    pub fn probe_all(&mut self) -> usize {
        for driver in &mut self.drivers {
            driver.state = DriverState::Probed;
        }
        self.drivers.len()
    }

    pub fn count(&self) -> usize {
        self.drivers.len()
    }
}
