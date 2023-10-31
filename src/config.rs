use crate::{error::HperwasmError, host};

pub struct ProcessConfig(ProcessConfigType);

enum ProcessConfigType {
    Config(u64),
    Inherit,
}

impl ProcessConfig {
    pub fn new() -> Result<Self, HperwasmError> {
        match unsafe {
            host::api::process::create_config()
        } {
            -1 => Err(HperwasmError::PermissionDenied),
            id => Ok(Self(ProcessConfigType::Config(id as u64))),
        }
    }

    pub(crate) fn inherit() -> Self {
        Self(ProcessConfigType::Inherit)
    }

    pub fn id(&self) -> i64 {
        match self.0 {
            ProcessConfigType::Config(id) => id as i64,
            ProcessConfigType::Inherit => -1,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe {
            host::api::process::config_set_name(self.id() as u64, name.as_ptr(), name.len(),)
        }
    }

    pub fn set_expected_time(&mut self, time: u64) {
        unsafe {
            host::api::process::config_set_expected_time(self.id() as u64, time)
        }
    }

    pub fn set_relative_ddl(&mut self, time: u64) {
        unsafe {host::api::process::config_set_relative_ddl(self.id() as u64, time)}
    }

}