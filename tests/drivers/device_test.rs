extern crate pos_kernel;

use pos_kernel::drivers::driver::{Driver, DriverRegistry, DriverState};

struct TestDriver {
    name: &'static str,
    state: DriverState,
}

impl Driver for TestDriver {
    fn name(&self) -> &'static str {
        self.name
    }

    fn probe(&mut self) -> Result<(), &'static str> {
        self.state = DriverState::Probed;
        Ok(())
    }

    fn state(&self) -> DriverState {
        self.state
    }
}

#[test]
fn driver_registry_registers_unique_devices() {
    let mut registry = DriverRegistry::new();

    assert!(registry.register("nvme"));
    assert!(registry.register("gpu"));
    assert!(!registry.register("nvme"));
    assert_eq!(registry.count(), 2);
}

#[test]
fn driver_registry_marks_registered_drivers_as_probed() {
    let mut registry = DriverRegistry::new();
    registry.register("serial");

    registry.probe("serial").unwrap();

    assert_eq!(registry.state("serial"), Some(DriverState::Probed));
}

#[test]
fn driver_trait_tracks_probe_state() {
    let mut driver = TestDriver {
        name: "keyboard",
        state: DriverState::New,
    };

    assert_eq!(driver.name(), "keyboard");
    assert_eq!(driver.state(), DriverState::New);

    driver.probe().unwrap();
    assert_eq!(driver.state(), DriverState::Probed);
}
