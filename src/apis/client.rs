use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    one_click_api: Box<dyn crate::apis::OneClickApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            one_click_api: Box::new(crate::apis::OneClickApiClient::new(rc.clone())),
        }
    }

    pub fn one_click_api(&self) -> &dyn crate::apis::OneClickApi{
        self.one_click_api.as_ref()
    }

}
