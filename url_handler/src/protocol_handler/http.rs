use super::ProtocolHandler;
use std::collections::HashSet;
use anyhow::Result;
use reqwest::{
    blocking::{Client, RequestBuilder},
    header::HeaderMap,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct HttpProtocolHandlerConfig {
    hosts: Option<HashMap<String, HostConfig>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    Delete,
    Get,
    Head,
    Patch,
    Post,
    Put,
}

impl HttpMethod {
    pub fn to_request(&self, url: &Url) -> RequestBuilder {
        let client = Client::new();
        let request = match self {
            HttpMethod::Delete => client.delete(url.as_str()),
            HttpMethod::Get => client.get(url.as_str()),
            HttpMethod::Head => client.head(url.as_str()),
            HttpMethod::Patch => client.patch(url.as_str()),
            HttpMethod::Post => client.post(url.as_str()),
            HttpMethod::Put => client.put(url.as_str()),
        };
        request
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
struct HostConfig {
    headers: Option<HashMap<String, String>>,
    push_method: Option<HttpMethod>,
    fetch_method: Option<HttpMethod>,
    bearer: Option<String>,
    user: Option<String>,
    password: Option<String>,
}

#[derive(Default)]
pub struct HttpProtocolHandler {
    config_per_host: HashMap<String, HostConfig>,
}

impl HttpProtocolHandler {
    pub fn new(config: &HttpProtocolHandlerConfig) -> Self {
        HttpProtocolHandler {
            config_per_host: match &config.hosts {
                None => HashMap::default(),
                Some(map) => map.clone(),
            },
        }
    }

    fn build_request_with_config(
        &self,
        url: &Url,
        default_method: HttpMethod,
    ) -> Result<RequestBuilder> {
        let Some(host) = url.host() else {
            anyhow::bail!("Could not extract host from url");
        };
        let Some(config) = self.config_per_host.get(&format!("{}", host)) else {
            return Ok(default_method.to_request(url));
        };

        let method = match &config.fetch_method {
            Some(method) => method,
            None => &default_method,
        };
        let request = method.to_request(url);

        let request = match &config.user {
            Some(user) => request.basic_auth(user, config.password.clone()),
            None => request,
        };
        let request = match &config.bearer {
            Some(token) => request.bearer_auth(token),
            None => request,
        };
        let request = match &config.headers {
            None => request,
            Some(headers) => {
                let map = HeaderMap::try_from(headers)?;
                request.headers(map.clone())
            }
        };

        Ok(request)
    }
}

impl ProtocolHandler for HttpProtocolHandler {
    fn fetch_string_from_url(&self, url: &Url) -> Result<Option<String>> {
        let request = self.build_request_with_config(url, HttpMethod::Get)?;
        let response = request.send()?.error_for_status()?;
        let string = response.text()?;
        Ok(Some(string))
    }

    fn push_string_to_url(&self, url: &Url, string: &str) -> Result<()> {
        self.build_request_with_config(url, HttpMethod::Put)?
            .body(string.to_string().clone())
            .send()?
            .error_for_status()?;
        Ok(())
    }

    fn delete_string_from_url(&self, _: &Url) -> Result<()> {
        todo!("Delete Operation is not yet implemented for the http handler!")
    }
    fn create_empty_string_on_url(&self, _: &Url) -> Result<()> {
        todo!("Create String Operation is not yet implemented for the http handler!")
    }
    fn create_url_container(&self, _: &Url) -> Result<()> {
        todo!("Create Container Container Operation is not yet implemented for the http handler!")
    }
    fn list_urls_in_url_container(&self, _: &Url) -> Result<HashSet<Url>> {
        todo!("List URL Operation is not yet implemented for the http handler!")
    }
}
