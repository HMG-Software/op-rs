#![allow(unreachable_code)]

#[cfg(feature = "config")]
use crate::CONFIG;

pub(crate) fn user() -> &'static str {
    #[cfg(feature = "config")]
    {
        return CONFIG
            .get()
            .unwrap()
            .user_id
            .unwrap()
            .as_ref();
    }

    "me"
}
