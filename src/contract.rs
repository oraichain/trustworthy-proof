#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Proof, CONFIG, PROOFS};

const DEFAULT_BASE_IPFS: &str = "https://node1-gateway-ipfs.eueno.io/ipfs";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CONFIG.save(
        deps.storage,
        &Config {
            owner: deps.api.addr_canonicalize(info.sender.as_str())?,
            base_ipfs: msg.base_ipfs.unwrap_or(DEFAULT_BASE_IPFS.to_string()),
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig { owner, base_ipfs } => {
            update_config(deps, info, owner, base_ipfs)
        }
        ExecuteMsg::UpdateProof {
            report_hash,
            ai_provider,
        } => update_proof(deps, env, info, report_hash, ai_provider),
    }
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Addr,
    base_ipfs: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.owner != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(ContractError::Unauthorized {});
    }

    config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    if let Some(base_ipfs) = base_ipfs {
        config.base_ipfs = base_ipfs;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn update_proof(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    report_hash: String,
    ai_provider: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.owner != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(ContractError::Unauthorized {});
    }

    let proof = Proof {
        created_time: env.block.time.seconds(),
        ai_provider,
        report_link: format!("{}/{}", config.base_ipfs, report_hash),
    };
    PROOFS.save(deps.storage, report_hash.as_bytes(), &proof)?;

    Ok(Response::new().add_attribute("action", "update_proof"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Proof { report_hash } => to_binary(&query_proof(deps, report_hash)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let Config { owner, base_ipfs } = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&owner)?,
        base_ipfs,
    })
}

pub fn query_proof(deps: Deps, report_hash: String) -> StdResult<Proof> {
    PROOFS.load(deps.storage, report_hash.as_bytes())
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
    };

    use crate::{
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
        state::Proof,
    };

    use super::{execute, instantiate, query};

    #[test]
    fn simple_case() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { base_ipfs: None };
        let info = mock_info("owner", &[]);

        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::UpdateProof {
            report_hash: "QmPTWAT6ySZ5LWEK736TBLZjZnpkHG9PHc2wtTpH7rJ26L".to_string(),
            ai_provider: "airi".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let res: Proof = from_binary(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::Proof {
                    report_hash: "QmPTWAT6ySZ5LWEK736TBLZjZnpkHG9PHc2wtTpH7rJ26L".to_string(),
                },
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(res.report_link,"https://node1-gateway-ipfs.eueno.io/ipfs/QmPTWAT6ySZ5LWEK736TBLZjZnpkHG9PHc2wtTpH7rJ26L");
    }
}
