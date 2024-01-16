use super::{DomainToken, DomainTokenType, SubgraphSettings};
use crate::entity::subgraph::domain::DetailedDomain;
use anyhow::Context;
use bigdecimal::{num_bigint::BigInt, Num};
use ethers::types::Address;
use std::str::FromStr;

#[tracing::instrument(
    level = "info",
    skip(domain, subgraph_settings),
    fields(domain_name = domain.name),
    err,
)]
pub fn extract_tokens_from_domain(
    domain: &DetailedDomain,
    subgraph_settings: &SubgraphSettings,
) -> Result<Vec<DomainToken>, anyhow::Error> {
    let mut tokens = vec![];

    if let Some(contract) = subgraph_settings.native_token_contract {
        let is_second_level_domain = domain
            .name
            .as_ref()
            .map(|name| name.matches('.').count() == 1)
            .unwrap_or(true);
        // native NFT exists only if domain is second level (like abc.eth and not abc.abc.eth)
        if is_second_level_domain {
            let labelhash = domain
                .labelhash
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("no labelhash in database"))?;

            let id = token_id(&hex::encode(labelhash))?;
            tokens.push(DomainToken {
                id,
                contract,
                _type: DomainTokenType::Native,
            });
        }
    };

    if domain.wrapped_owner.is_some() {
        let id = token_id(&domain.id)?;
        let contract = Address::from_str(&domain.owner).context("parse owner as address")?;
        tokens.push(DomainToken {
            id,
            contract,
            _type: DomainTokenType::Wrapped,
        });
    };

    Ok(tokens)
}

fn token_id(hexed_id: &str) -> Result<String, anyhow::Error> {
    let id = BigInt::from_str_radix(hexed_id.trim_start_matches("0x"), 16)
        .context("convert token_id to number")?;
    Ok(id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[inline]
    fn domain(
        name: &str,
        id: &str,
        labelhash: &str,
        owner: &str,
        maybe_wrapped_owner: Option<&str>,
    ) -> DetailedDomain {
        DetailedDomain {
            id: id.to_string(),
            name: Some(name.to_string()),
            labelhash: Some(
                hex::decode(labelhash.trim_start_matches("0x"))
                    .expect("invalid labelhash provided"),
            ),
            owner: owner.to_string(),
            wrapped_owner: maybe_wrapped_owner.map(str::to_string),
            ..Default::default()
        }
    }

    #[inline]
    fn addr(a: &str) -> Option<Address> {
        Address::from_str(a).ok()
    }

    #[test]
    fn it_works() {
        let native_contract = "0x1234567890123456789012345678901234567890";
        let wrapped_contract = "0x0987654321098765432109876543210987654321";
        let owner = "0x1111111111111111111111111111111111111111";
        for (domain, native_token_contract, expected_tokens) in [
            (
                domain("levvv.eth", "0x0200", "0x0100", owner, None),
                addr(native_contract),
                vec![DomainToken {
                    id: "256".to_string(),
                    contract: Address::from_str(native_contract)
                        .expect("invalid native_contract provided"),
                    _type: DomainTokenType::Native,
                }],
            ),
            (
                domain(
                    "levvv.eth",
                    "0x0200",
                    "0x0100",
                    wrapped_contract,
                    Some(owner),
                ),
                addr(native_contract),
                vec![
                    DomainToken {
                        id: "256".to_string(),
                        contract: Address::from_str(native_contract)
                            .expect("invalid native_contract provided"),
                        _type: DomainTokenType::Native,
                    },
                    DomainToken {
                        id: "512".to_string(),
                        contract: Address::from_str(wrapped_contract)
                            .expect("invalid wrapped_contract provided"),
                        _type: DomainTokenType::Wrapped,
                    },
                ],
            ),
        ] {
            let settings = SubgraphSettings {
                native_token_contract,
                ..Default::default()
            };
            let tokens = extract_tokens_from_domain(&domain, &settings)
                .expect("failed to extract tokens from domain");

            assert_eq!(tokens, expected_tokens);
        }
    }
}
