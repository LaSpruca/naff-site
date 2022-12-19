use rand::{thread_rng, Rng};
use serde::Deserialize;
use tracing::info;

use crate::error::*;

#[derive(Deserialize, Clone)]
pub struct UrlConfig {
    pub backend: String,
    pub frontend: String,
}

#[derive(Deserialize, Clone)]
pub struct Auth0Config {
    pub client_id: String,
    pub client_secret: String,
    pub domain: String,
}

pub fn get_config() -> Result<(Auth0Config, UrlConfig), Error> {
    let authz_config: Auth0Config = envy::prefixed("AUTH0_").from_env().to_crate()?;
    let public_config: UrlConfig = envy::prefixed("PUBLIC_").from_env().to_crate()?;

    Ok((authz_config, public_config))
}

#[derive(Clone, Debug)]
pub struct States {
    inner: Vec<String>,
}

impl States {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn add(&mut self) -> String {
        let mut rng = thread_rng();
        let state: String = (0..10)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect();
        self.inner.push(state.clone());

        state
    }

    pub fn check(&mut self, state: &String) -> bool {
        info!("{state} {:?}", self.inner);
        let find_index = self.inner.iter().position(|x| x == state);
        if let Some(idex) = find_index {
            self.inner.remove(idex);
            return true;
        }

        false
    }
}
