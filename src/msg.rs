use cosmwasm_schema::{cw_serde, QueryResponses};
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::State;

#[cw_serde]
pub struct InstantiateMsg {
    pub peer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub contract: String,
    pub token_id: String,
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Swap {},
    Withdraw {
        contract: String,
        token_id: String,
    },
    ReceiveNft(Cw721ReceiveMsg)
}

#[cw_serde]
pub enum InnerMsg {
    Deposit { token_id: String, contract: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ParticipantsResponse {
    pub creator: String,
    pub peer: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(State)]
    State {},

    #[returns(Vec<Token>)]
    Tokens {},

    #[returns(bool)]
    Ended {},

    #[returns(ParticipantsResponse)]
    Participants {}
}
