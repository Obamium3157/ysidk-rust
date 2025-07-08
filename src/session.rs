use reqwest::{blocking, header::HeaderValue};

use crate::funcitons::Res;

const AUTHORIZATION_TOKEN: &'static str = "y0__xC0qIb_Axi85zgg4sDE4BOBkr9aha6pKslrohTx-3ivfnxOHw";

pub struct Session {
    client: blocking::Client,
    auth_header: HeaderValue
}

impl Session {
    pub fn new() -> Res<Self> {
        Ok(Self {
            client: blocking::Client::new(),
            auth_header: HeaderValue::from_str(&format!("OAuth {}", AUTHORIZATION_TOKEN))?
        })
    }

    pub fn client(&self) -> &blocking::Client {
        &self.client
    }

    pub fn auth_header(&self) -> &HeaderValue {
        &self.auth_header
    }

    pub fn get_pair(&self) -> (&blocking::Client, &HeaderValue) {
        (&self.client, &self.auth_header)
    }
}