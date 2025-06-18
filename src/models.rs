use std::fmt;

use serde::{Deserialize, Serialize};

pub mod auth;
pub mod common;
pub mod orders;

macro_rules! id_newtype {
    ($(($name:ident, $base_type:ty)),+) => {$(
        #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        pub struct $name(pub $base_type);

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl $name {
            pub fn into_inner(self) -> $base_type {
                self.0
            }
        }

        impl From<$base_type> for $name {
            fn from(value: $base_type) -> Self {
                Self(value)
            }
        }

        impl AsRef<$base_type> for $name {
            fn as_ref(&self) -> &$base_type {
                &self.0
            }
        }
    )+};
}

id_newtype!(
    (ClientId, String),
    (GoodsOwnerId, u32),
    (OrderNumber, String),
    (ReferenceNumber, String),
    (CustomerNumber, String)
);

// Almost the same as the ids, but doing this one manually since it's not an id and will probably
// need validation soon.
// TODO: Validation: must be > 0. Use NonZeroU32?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCount(u32);

impl fmt::Display for ItemCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ItemCount({})", self.0)
    }
}

impl ItemCount {
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl From<u32> for ItemCount {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl AsRef<u32> for ItemCount {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
