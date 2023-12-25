#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, from_binary, to_binary, WasmMsg::Execute};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, InnerMsg, Token, ParticipantsResponse};
use crate::state::{State, STATE};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:p2p";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        creator: _info.sender.to_string(),
        peer: msg.peer,
        tokens: Vec::new(),
        ended: false,
        creator_signed: false,
        peer_signed: false,
    };
    STATE.save(_deps.storage, &state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ReceiveNft(msg) => {
            let inner: InnerMsg = from_binary(&msg.msg)?;
            match inner {
                InnerMsg::Deposit { token_id, contract } => {
                    let mut state = STATE.load(_deps.storage)?;
                    state.tokens.push(Token { token_id, contract, owner: info.sender.to_string() });
                }
                _ => return Err(ContractError::Unauthorized {})
            }
            Ok(Response::default())
        },
        ExecuteMsg::Swap { } => {
            let mut state = STATE.load(_deps.storage)?;
            if info.sender.to_string() == state.creator {
                state.creator_signed = true;
            } else if info.sender.to_string() == state.peer {
                state.peer_signed = true;
            } else {
                return Err(ContractError::Unauthorized {})
            }

            if state.creator_signed && state.peer_signed {
                let mut resp: Response = Response::new();
                for token in state.tokens {
                    resp = resp.add_message(Execute { 
                        contract_addr: token.contract, 
                        msg: to_binary(
                            &cw721::Cw721ExecuteMsg::TransferNft { 
                                recipient: if &token.owner == &state.creator {String::from(&state.peer)} else {String::from(&state.creator)}, 
                                token_id: token.token_id
                            }
                        )?, 
                        funds: vec![] 
                    });
                }
                state.ended = true;
                Ok(resp)
            }else {
                STATE.save(_deps.storage, &state)?;
                Ok(Response::default())
            }
        },
        ExecuteMsg::Withdraw { contract, token_id } => {
            let state = STATE.load(_deps.storage)?;

            if state.tokens.contains(&Token { token_id: token_id.clone(), contract: contract.clone(), owner: info.sender.to_string() }) != true {
                return Err(ContractError::Unauthorized {})
            }

            Ok(
                Response::new()
                .add_message(Execute { 
                    contract_addr: contract, 
                    msg: to_binary(
                        &cw721::Cw721ExecuteMsg::TransferNft { 
                            recipient: info.sender.into_string(), 
                            token_id: token_id 
                        }
                    )?, 
                    funds: vec![] 
                })
            )
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::State {  } => Ok(to_binary(&STATE.load(_deps.storage)?)?),
        QueryMsg::Tokens {  } => Ok(to_binary(&STATE.load(_deps.storage)?.tokens)?),
        QueryMsg::Ended {  } => Ok(to_binary(&STATE.load(_deps.storage)?.ended)?),
        QueryMsg::Participants {  } => Ok(to_binary(&ParticipantsResponse {
            creator: STATE.load(_deps.storage)?.creator,
            peer: STATE.load(_deps.storage)?.peer,
        })?),
    }
}
