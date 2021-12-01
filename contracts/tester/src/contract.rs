use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cudos_cosmwasm::{create_issue_denom_msg, CudosMsgWrapper, CudosQuerier, DenomResponse};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<CudosMsgWrapper>> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<CudosMsgWrapper>, StdError> {
    match msg {
        ExecuteMsg::IssueDenomMsg {
            id,
            name,
            schema,
            sender,
        } => execute_msg_issue_denom(deps, env, info, id, name, schema, sender),
    }
}

pub fn execute_msg_issue_denom(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    id: String,
    name: String,
    schema: String,
    sender: String,
) -> StdResult<Response<CudosMsgWrapper>> {
    let msg = create_issue_denom_msg(id, name, schema, sender);

    Ok(Response::new().add_message(msg))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Denom { denom_id } => to_binary(&query_denom(deps, denom_id)?),
    }
}

pub fn query_denom(deps: Deps, denom_id: String) -> StdResult<DenomResponse> {
    let querier = CudosQuerier::new(&deps.querier);
    let res: DenomResponse = querier.query_denom(denom_id)?;

    Ok(res)
}
