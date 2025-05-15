use crate::{error::Result, models::Credentials};

pub trait CredentialProvider {
    fn try_fetch_credentials(&self) -> Result<Credentials>;
}
