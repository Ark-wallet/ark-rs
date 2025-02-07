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
pub struct V1RedeemTransaction {
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "spentVtxos", skip_serializing_if = "Option::is_none")]
    pub spent_vtxos: Option<Vec<models::V1Outpoint>>,
    #[serde(rename = "spendableVtxos", skip_serializing_if = "Option::is_none")]
    pub spendable_vtxos: Option<Vec<models::V1Vtxo>>,
}

impl V1RedeemTransaction {
    pub fn new() -> V1RedeemTransaction {
        V1RedeemTransaction {
            txid: None,
            spent_vtxos: None,
            spendable_vtxos: None,
        }
    }
}
