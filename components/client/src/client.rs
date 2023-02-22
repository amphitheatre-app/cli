// Copyright 2022 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ureq::Request;

use super::accounts::Accounts;
use super::actors::Actors;
use super::errors::ClientError;
use super::oauth::OAuth;
use super::playbooks::Playbooks;

const VERSION: &str = "0.1.0";
const DEFAULT_USER_AGENT: &str = "amp/";

const API_VERSION: &str = "v1";

/// Represents the Rust client for the Amphitheatre API
///
/// The client is your entrypoint to the Amphitheatre API. Using it you will be
/// able to call all the enpoints of the Amphitheatre API and their respective functions.
///
/// # Examples
///
/// ```no_run
/// use client::client::Client;
///
/// let client = Client::new(
///     String::from("https://cloud.amphitheatre.app"),
///     String::from("AUTH_TOKEN"),
/// );
/// let response = client.accounts().me().unwrap();
///
/// let account = response.data.unwrap();
/// ```
pub struct Client {
    base_url: String,
    user_agent: String,
    auth_token: String,
    pub _agent: ureq::Agent,
}

/// Defines the Endpoint trait for the different API endpoints
pub trait Endpoint {
    type Output: DeserializeOwned;
}

/// Represents the response from an API call
#[derive(Debug)]
pub struct Response<T> {
    /// The maximum number of requests you can perform per hour.
    pub rate_limit: String,
    /// The number of requests remaining in the current rate limit window.
    pub rate_limit_remaining: String,
    /// The time at which the current rate limit window in [Unix
    /// time](https://en.wikipedia.org/wiki/Unix_time) format.
    pub rate_limit_reset: Option<String>,
    /// The HTTP Status Code
    pub status: u16,
    /// The object or a Vec<T> objects (the type `T` will depend on the endpoint).
    pub data: Option<T>,
    /// Any API endpoint that returns a list of items requires pagination.
    pub pagination: Option<Pagination>,
    /// The body as a JSON `Value`
    pub body: Option<Value>,
}

/// Any API endpoint that returns a list of items requires pagination.
/// By default we will return 30 records from any listing endpoint. If an API
/// endpoint returns a list of items, then it will include a pagination object
/// that contains pagination information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Pagination {
    /// The page currently returned (default: 1)
    pub current_page: u64,
    /// The number of entries returned per page (default: 30)
    pub per_page: u64,
    /// The Total number of entries available in the entrire collection.
    pub total_entries: u64,
    /// The total number of pages available given the current `per_page` value
    pub total_pages: u64,
}

/// When you can send some options into the request (i.e. for pagination).
pub struct RequestOptions {
    /// Filtering makes it possible to ask only for the exact subset of data
    /// that you're looking for.
    pub filters: Option<Filters>,
    /// API results are implicitly sorted according to policies that very from
    /// endpoint to endpoint.
    pub sort: Option<Sort>,
    /// Pagination options
    pub paginate: Option<Paginate>,
}

/// Represents an empty response from the Amphitheatre API
/// (_these type of response happen when issuing DELETE commands for example_)
pub struct EmptyResponse {
    /// The maximum number of requests you can perform per hour.
    pub rate_limit: String,
    /// The number of requests remaining in the current rate limit window.
    pub rate_limit_remaining: String,
    /// The time at which the current rate limit window in [Unix time](https://en.wikipedia.org/wiki/Unix_time) format.
    pub rate_limit_reset: Option<String>,
    /// The HTTP Status Code
    pub status: u16,
}

/// Filtering makes it possible to ask only for exact subset of data that you're
/// looking for.
///
/// With potential hundreds of result entries, it's convenient to apply a filter
/// and receive only the interesting data.
#[derive(Debug)]
pub struct Filters {
    pub filters: HashMap<String, String>,
}

impl Filters {
    pub fn new(filters: HashMap<String, String>) -> Filters {
        Filters { filters }
    }
}

/// API results are implicitly sorted according to policies that very from
/// endpoint to endpoint.
///
/// You can decide your own sorting policy for each single API call via the sort paramter.
/// this paramter accepts a set of comma separated key-value pairs: the name of
/// a field and the order criteria (asc for ascending and desc for descending).
///
/// The order of fields is relevant, as it will determine the priority of the
/// sorting policies.
#[derive(Debug)]
pub struct Sort {
    pub sort_by: String,
}

impl Sort {
    pub fn new(sort_by: String) -> Sort {
        Sort { sort_by }
    }
}

/// The pagination instructions for the request
pub struct Paginate {
    /// The number of items you want
    pub per_page: u32,
    /// The page number
    pub page: u32,
}

impl Client {
    /// Helper function to create a new client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use client::client::Client;
    /// let client = Client::new(
    ///     String::from("https://cloud.amphitheatre.app"),
    ///     String::from("AUTH_TOKEN"),
    /// );
    /// ```
    ///
    /// # Arguments
    ///
    /// `token`: the bearer authentication token
    pub fn new(base_url: String, token: String) -> Client {
        Client {
            base_url,
            user_agent: DEFAULT_USER_AGENT.to_owned() + VERSION,
            auth_token: token,
            _agent: ureq::Agent::new(),
        }
    }

    /// Returns the current url (including the `API_VERSION` as part of the path).
    pub fn versioned_url(&self) -> String {
        let mut url = String::from(&self.base_url);
        url.push('/');
        url.push_str(API_VERSION);
        url
    }
}

impl Client {
    /// Sends a GET request to the Amphitheatre API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `options`: optionally a `RequestOptions` with things like pagination,
    /// filtering and sorting
    pub fn get<E: Endpoint>(
        &self,
        path: &str,
        options: Option<RequestOptions>,
    ) -> Result<Response<E::Output>, ClientError> {
        self.call::<E>(self.build_get_request(&path, options))
    }

    /// Sends a POST request to the Amphitheatre API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn post<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<Response<<E as Endpoint>::Output>, ClientError> {
        self.call_with_payload::<E>(self.build_post_request(&path), data)
    }

    /// Sends a POST request to the Amphitheatre API without any payload
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn empty_post(&self, path: &str) -> Result<EmptyResponse, ClientError> {
        self.call_empty(self.build_post_request(&path))
    }

    /// Sends a PUT request to the Amphitheatre API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn put<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<Response<<E as Endpoint>::Output>, ClientError> {
        self.call_with_payload::<E>(self.build_put_request(&path), data)
    }

    /// Sends a PUT request to the Amphitheatre API without any payload
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn empty_put(&self, path: &str) -> Result<EmptyResponse, ClientError> {
        self.call_empty(self.build_put_request(&path))
    }

    /// Sends a PATCH request to the Amphitheatre API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    /// `data`: the json payload to be sent to the server
    pub fn patch<E: Endpoint>(
        &self,
        path: &str,
        data: Value,
    ) -> Result<Response<<E as Endpoint>::Output>, ClientError> {
        self.call_with_payload::<E>(self.build_patch_request(&path), data)
    }

    /// Sends a DELETE request to the Amphitheatre API
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn delete(&self, path: &str) -> Result<EmptyResponse, ClientError> {
        self.call_empty(self.build_delete_request(&path))
    }

    /// Sends a DELETE request to the Amphitheatre API returning a response containing a `Response`
    ///
    /// # Arguments
    ///
    /// `path`: the path to the endpoint
    pub fn delete_with_response<E: Endpoint>(&self, path: &str) -> Result<Response<E::Output>, ClientError> {
        self.call::<E>(self.build_delete_request(&path))
    }

    fn call_with_payload<E: Endpoint>(
        &self,
        request: Request,
        data: Value,
    ) -> Result<Response<E::Output>, ClientError> {
        self.process_response::<E>(request.send_json(data))
    }

    fn call<E: Endpoint>(&self, request: Request) -> Result<Response<E::Output>, ClientError> {
        self.process_response::<E>(request.call())
    }

    fn process_response<E: Endpoint>(
        &self,
        result: Result<ureq::Response, ureq::Error>,
    ) -> Result<Response<E::Output>, ClientError> {
        match result {
            Ok(response) => Self::build_response::<E>(response),
            Err(ureq::Error::Status(code, response)) => Err(ClientError::parse_response(code, response)),
            Err(ureq::Error::Transport(transport)) => Err(ClientError::parse_transport(transport)),
        }
    }

    fn call_empty(&self, request: Request) -> Result<EmptyResponse, ClientError> {
        match request.call() {
            Ok(response) => Self::build_empty_response(response),
            Err(ureq::Error::Status(code, response)) => Err(ClientError::parse_response(code, response)),
            Err(ureq::Error::Transport(transport)) => Err(ClientError::parse_transport(transport)),
        }
    }

    fn build_response<E: Endpoint>(resp: ureq::Response) -> Result<Response<E::Output>, ClientError> {
        let rate_limit = Self::extract_rate_limit_limit_header(&resp)?;
        let rate_limit_remaining = Self::extract_rate_limit_remaining_header(&resp)?;
        let rate_limit_reset = Self::extract_rate_limit_reset_header(&resp);

        let status = resp.status();

        let json = resp
            .into_json::<Value>()
            .map_err(|e| ClientError::Deserialization(e.to_string()))?;
        let data = serde_json::from_value(json!(json.get("data")))
            .map_err(|e| ClientError::Deserialization(e.to_string()))?;
        let pagination = serde_json::from_value(json!(json.get("pagination")))
            .map_err(|e| ClientError::Deserialization(e.to_string()))?;
        let body = serde_json::from_value(json).map_err(|e| ClientError::Deserialization(e.to_string()))?;

        Ok(Response {
            rate_limit,
            rate_limit_remaining,
            rate_limit_reset,
            status,
            data,
            pagination,
            body,
        })
    }

    fn extract_rate_limit_reset_header(resp: &ureq::Response) -> Option<String> {
        resp.header("x-ratelimit-after").map(|header| header.to_string())
    }

    fn extract_rate_limit_remaining_header(resp: &ureq::Response) -> Result<String, ClientError> {
        match resp.header("x-ratelimit-remaining") {
            Some(header) => Ok(header.to_string()),
            None => Err(ClientError::Deserialization(String::from(
                "Cannot parse the x-ratelimit-remaining header",
            ))),
        }
    }

    fn extract_rate_limit_limit_header(resp: &ureq::Response) -> Result<String, ClientError> {
        match resp.header("x-ratelimit-limit") {
            Some(header) => Ok(header.to_string()),
            None => Err(ClientError::Deserialization(String::from(
                "Cannot parse the x-ratelimit-limit header",
            ))),
        }
    }

    fn build_empty_response(response: ureq::Response) -> Result<EmptyResponse, ClientError> {
        Ok(EmptyResponse {
            rate_limit: Self::extract_rate_limit_limit_header(&response)?,
            rate_limit_remaining: Self::extract_rate_limit_remaining_header(&response)?,
            rate_limit_reset: Self::extract_rate_limit_reset_header(&response),
            status: response.status(),
        })
    }

    fn build_get_request(&self, path: &&str, options: Option<RequestOptions>) -> Request {
        let mut request = self
            ._agent
            .get(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");

        if let Some(options) = options {
            if let Some(pagination) = options.paginate {
                request = request.query("page", &pagination.page.to_string());
                request = request.query("per_page", &pagination.per_page.to_string())
            }

            if let Some(filters) = options.filters {
                for (key, value) in filters.filters {
                    request = request.query(&key, &value);
                }
            }

            if let Some(sort) = options.sort {
                request = request.query("sort", &sort.sort_by);
            }
        }

        self.add_headers_to_request(request)
    }

    pub fn build_post_request(&self, path: &&str) -> Request {
        let request = self
            ._agent
            .post(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    pub fn build_put_request(&self, path: &&str) -> Request {
        let request = self
            ._agent
            .put(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    pub fn build_patch_request(&self, path: &&str) -> Request {
        let request = self
            ._agent
            .request("PATCH", &self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    fn build_delete_request(&self, path: &&str) -> Request {
        let request = self
            ._agent
            .delete(&self.url(path))
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/json");
        self.add_headers_to_request(request)
    }

    fn add_headers_to_request(&self, request: Request) -> Request {
        let auth_token = &format!("Bearer {}", self.auth_token);
        request.set("Authorization", auth_token.as_str())
    }

    pub fn url(&self, path: &str) -> String {
        let mut url = self.versioned_url();
        url.push_str(path);

        println!("url = {}", url);
        url
    }
}

#[cfg(test)]
mod tests {
    use crate::client::{Client, DEFAULT_USER_AGENT, VERSION};
    const BASE_URL: &str = "https://cloud.amphitheatre.app";

    #[test]
    fn creates_a_client() {
        let token = "some-auth-token";
        let client = Client::new(String::from(BASE_URL), String::from(token));

        assert_eq!(client.base_url, BASE_URL);
        assert_eq!(client.user_agent, DEFAULT_USER_AGENT.to_owned() + VERSION);
        assert_eq!(client.auth_token, token);
    }
}

impl Client {
    /// Returns the `accounts` services attached to this client
    pub fn accounts(&self) -> Accounts {
        Accounts { client: self }
    }

    /// Returns the `actors` services attached to this client
    pub fn actors(&self) -> Actors {
        Actors { client: self }
    }

    /// Returns the `oauth` service attached to this client
    pub fn oauth(&self) -> OAuth {
        OAuth { client: self }
    }

    /// Returns the `playbooks` service attached to this client
    pub fn playbooks(&self) -> Playbooks {
        Playbooks { client: self }
    }
}
