/*
 * ark/v1/service.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: version not set
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1RegisterInputsForNextRoundResponse {
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl V1RegisterInputsForNextRoundResponse {
    pub fn new() -> V1RegisterInputsForNextRoundResponse {
        V1RegisterInputsForNextRoundResponse { request_id: None }
    }
}
