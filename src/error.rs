use std::fmt::{Debug, Display, Formatter};

use thiserror::Error;

use crate::host::api::error;


#[derive(Error)]
pub enum HperwasmError {
    Error(u64),
    PermissionDenied,
}

impl Drop for HperwasmError {
    fn drop(&mut self) {
        match self {
            HperwasmError::Error(id) => {
                //unsafe { error::drop(*id) };
                println!("error happen in {:?}", id);
            }
            HperwasmError::PermissionDenied => (),
        }
    }
}

impl HperwasmError {
    pub(crate) fn from(id: u64) -> Self {
        HperwasmError::Error(id)
    }
}

impl Debug for HperwasmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            HperwasmError::Error(_id) => {
                // let size = unsafe { error::string_size(*id) };
                // let mut buff = vec![0; size as usize];
                // unsafe { error::to_string(*id, buff.as_mut_ptr()) };
                // let error = std::str::from_utf8(&buff).unwrap();
                let error = "test-error";
                write!(f, "{}", error)
            }
            HperwasmError::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}

impl Display for HperwasmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            HperwasmError::Error(_id) => {
                // let size = unsafe { error::string_size(*id) };
                // let mut buff = vec![0; size as usize];
                // unsafe { error::to_string(*id, buff.as_mut_ptr()) };
                // let error = std::str::from_utf8(&buff).unwrap();
                let error = "test-error";
                write!(f, "{}", error)
            }
            HperwasmError::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}
