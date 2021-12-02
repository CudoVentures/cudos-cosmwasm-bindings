use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    IssueDenomMsg {
        id: String,
        name: String,
        schema: String,
        sender: String,
    },
    MintNftMsg {
        denom_id: String,
        name: String,
        uri: String,
        data: String,
        sender: String,
        recipient: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryDenomById { denom_id: String },
    QueryDenomByName { denom_name: String },
    QueryToken { denom_id: String, token_id: String },
}