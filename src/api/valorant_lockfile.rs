use std::fmt::Display;
use std::{env, fs, io};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LockfileError {
    #[error("Failed to read lockfile: {0}")]
    ReadLockfileError(#[from] io::Error),

    #[error("Lockfile does not exist")]
    LockfileNotExist,

    #[error("Invalid lockfile content")]
    LockfileInvalid,
}

#[derive(Clone)]
pub struct Lockfile {
    pub name: String,
    pub pid: u32,
    pub port: u16,
    pub password: String,
    pub protocol: String,
}

impl Lockfile {
    pub fn new(name: String, pid: u32, port: u16, password: String, protocol: String) -> Self {
        Self { name, pid, port, password, protocol }
    }

    pub fn new_from_lockfile() -> Result<Self, LockfileError> {
        let local_app_data_path = env::var("LocalAppData")
            .map_err(|_| LockfileError::LockfileNotExist)?;

        let lockfile_path = PathBuf::from(local_app_data_path)
            .join("Riot Games")
            .join("Riot Client")
            .join("Config")
            .join("lockfile");

        if !lockfile_path.exists() {
            return Err(LockfileError::LockfileNotExist);
        }

        let lockfile_content = fs::read_to_string(lockfile_path)?;
        let lockfile_parts: Vec<&str> = lockfile_content.split(':').collect();

        if lockfile_parts.len() != 5 {
            return Err(LockfileError::LockfileInvalid);
        }

        let name = lockfile_parts.get(0).ok_or(LockfileError::LockfileInvalid)?.to_string();
        let pid = lockfile_parts.get(1).ok_or(LockfileError::LockfileInvalid)?.parse::<u32>()
            .map_err(|_| LockfileError::LockfileInvalid)?;
        let port = lockfile_parts.get(2).ok_or(LockfileError::LockfileInvalid)?.parse::<u16>()
            .map_err(|_| LockfileError::LockfileInvalid)?;
        let password = lockfile_parts.get(3).ok_or(LockfileError::LockfileInvalid)?.to_string();
        let protocol = lockfile_parts.get(4).ok_or(LockfileError::LockfileInvalid)?.to_string();

        Ok(Self::new(name, pid, port, password, protocol))
    }
}

impl Display for Lockfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}",
            self.name, self.pid, self.port, self.password, self.protocol
        )
    }
}
