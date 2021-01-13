// Copyright (c) 2018-2021 The MobileCoin Foundation

//! The Responder ID type

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use failure::Fail;
use mc_crypto_digestible::Digestible;
use serde::{Deserialize, Serialize};

/// Potential parse errors
#[derive(Debug, Fail, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum ResponderIdParseError {
    #[fail(display = "Failure from Utf8 for {:?}", _0)]
    FromUtf8Error(Vec<u8>),
    #[fail(display = "Invalid Format for {}", _0)]
    InvalidFormat(String),
}

/// Node unique identifier.
#[derive(
    Clone, Default, Debug, Eq, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Hash, Digestible,
)]
pub struct ResponderId(pub String);

impl Display for ResponderId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ResponderId {
    type Err = ResponderIdParseError;

    fn from_str(src: &str) -> Result<ResponderId, Self::Err> {
        // ResponderId is expected to be host:port, so at least ensure we have a single colon as a
        // small sanity test.
        if !src.contains(':') {
            return Err(ResponderIdParseError::InvalidFormat(src.to_string()));
        }

        Ok(Self(src.to_string()))
    }
}

// This is needed for SCPNetworkState's NetworkState implementation.
impl AsRef<ResponderId> for ResponderId {
    fn as_ref(&self) -> &Self {
        self
    }
}
