use thiserror::Error;
use crate::lockfile::{Lockfile};

#[derive(Error)]
enum LocalApiError {

}

pub struct LocalApi {
    lockfile: Lockfile,
}

impl LocalApi {
    pub fn new(lockfile: Lockfile) -> Self {
        Self { lockfile }
    }


}
