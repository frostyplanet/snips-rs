use std::prelude::v1::*;
use std::collections::HashMap;

pub struct APIBuilder {
    pub api_name: &'static str,
    pub request_method: &'static str,
    pub request_uri: &'static str,
    pub expect_status_code: [i16],
}

pub trait SnipsInput {

    fn get_headers(&self) -> Option<HashMap<&str, &str>>;

    fn get_params(&self)-> Option<HashMap<&str, &str>>;
}
