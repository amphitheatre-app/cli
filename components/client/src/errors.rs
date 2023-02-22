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

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use ureq::{Response, Transport};

/// Represents the possible errors thrown while interacting with the Amphitheatre API
#[derive(Error, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum ClientError {
    #[error("Authentication failed")]
    Unauthorized,

    #[error("Bad Gateway")]
    BadGateway,

    #[error("{message}")]
    BadRequest {
        message: String,
        attribute_errors: Option<Value>,
    },

    #[error("{0}")]
    GatewayTimeout(String),

    #[error("Method not Allowed")]
    MethodNotAllowed,

    #[error("{0}")]
    NotFound(String),

    #[error("Your account is not subscribed or ot in good stading")]
    PaymentRequired,

    #[error("{0}")]
    PreconditionRequired(String),

    #[error("Service Unavailable")]
    ServiceUnavailable,

    #[error("You exceeded the allowed number of requests per hour and your request has temporarily been throttled.")]
    TooManyRequests,

    #[error("Transport Error - {0}({1})")]
    Transport(String, String),

    #[error("Deserialization Error {0}")]
    Deserialization(String),
}

impl ClientError {
    pub fn parse_response(code: u16, response: Response) -> ClientError {
        match code {
            400 => Self::bad_request(response),
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            404 => Self::not_found(response),
            405 => Self::MethodNotAllowed,
            428 => Self::precondition_required(response),
            429 => Self::TooManyRequests,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::gateway_timeout(response),
            _ => Self::Transport(response.status().to_string(), response.status_text().to_string()),
        }
    }

    pub fn parse_transport(transport: Transport) -> ClientError {
        Self::Transport(transport.to_string(), transport.kind().to_string())
    }

    fn bad_request(response: Response) -> ClientError {
        match Self::response_to_json(response) {
            Ok(json) => Self::BadRequest {
                message: Self::message_in(&json),
                attribute_errors: Some(json["errors"].clone()),
            },
            Err(error) => error,
        }
    }

    fn gateway_timeout(response: Response) -> ClientError {
        match Self::response_to_json(response) {
            Ok(json) => Self::GatewayTimeout(Self::message_in(&json)),
            Err(error) => error,
        }
    }

    fn not_found(response: Response) -> ClientError {
        match Self::response_to_json(response) {
            Ok(json) => Self::NotFound(Self::message_in(&json)),
            Err(error) => error,
        }
    }

    fn precondition_required(response: Response) -> ClientError {
        match Self::response_to_json(response) {
            Ok(json) => Self::PreconditionRequired(Self::message_in(&json)),
            Err(error) => error,
        }
    }

    fn message_in(json: &Value) -> String {
        match json["message"].as_str() {
            None => String::from("Unable to parse error message"),
            Some(json_string) => json_string.to_string(),
        }
    }

    fn response_to_json(response: Response) -> Result<Value, ClientError> {
        match response.into_json::<Value>() {
            Ok(value) => Ok(value),
            Err(error) => Err(ClientError::Deserialization(error.to_string())),
        }
    }
}
