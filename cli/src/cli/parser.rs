#![allow(unreachable_code)]

use std::{env, path::PathBuf};

#[cfg(feature = "config")]
use crate::CONFIG;

pub(crate) fn token(token: &str) -> Result<String, String> {
    #[cfg(feature = "config")]
    {
        unimplemented!()
    }
}
