use cosmwasm_std::{Order, StdError, StdResult, Storage};
use cw_storage_plus::{Bound, Map, PrimaryKey};
use dymension_std::types::cosmos::base::query::v1beta1::{PageRequest, PageResponse};
use serde::{de::DeserializeOwned, Serialize};

pub fn paginate<'a, K, T>(
    storage: &dyn Storage,
    map: Map<'a, K, T>,
    req_opt: Option<PageRequest>,
    mut on_result: impl FnMut((Vec<u8>, T)) -> Option<StdError>,
) -> StdResult<PageResponse>
where
    T: Serialize + DeserializeOwned,
    K: PrimaryKey<'a>,
{
    let req = match req_opt {
        Some(p) => p,
        None => PageRequest::default(),
    };

    let offset = req.offset;
    let key = req.key;
    let mut limit = req.limit;
    let mut count_total = req.count_total;
    let reverse = req.reverse;

    if offset > 0 && key.len() > 0 {
        return Err(StdError::generic_err(
            "invalid request, either offset or key is expected, got both",
        ));
    }

    if limit == 0 {
        limit = 100; // default limit
        count_total = true;
    }

    let order = match reverse {
        true => Order::Descending,
        false => Order::Ascending,
    };

    if key.len() != 0 {
        let mut count: u64 = 0;
        let mut next_key: Vec<u8> = vec![];

        let min: Option<Bound<'a, K>> = match reverse {
            true => None,
            false => Some(Bound::InclusiveRaw(key.clone())),
        };

        let max: Option<Bound<'a, K>> = match reverse {
            true => Some(Bound::InclusiveRaw(key)),
            false => None,
        };

        for result in map.range_raw(storage, min, max, order) {
            if count == limit {
                next_key = result?.0.clone();
                break;
            }
            let err = on_result(result?);
            if let Some(e) = err {
                return Err(e);
            }
            count += 1;
        }

        return Ok(PageResponse {
            next_key: next_key,
            total: 0,
        });
    }

    let end = offset + limit;
    let mut count: u64 = 0;
    let mut next_key: Vec<u8> = vec![];

    for result in map.range_raw(storage, None, None, order) {
        count += 1;

        if count <= offset {
            continue;
        }

        if count <= end {
            let err = on_result(result?);
            if let Some(e) = err {
                return Err(e);
            }
        } else if count == end + 1 {
            next_key = result?.0;

            if !count_total {
                break;
            }
        }
    }

    let mut res = PageResponse {
        next_key: next_key,
        total: 0,
    };
    if count_total {
        res.total = count;
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn test_paginate() {
        let mut deps = mock_dependencies();
        let test_state: Map<Vec<u8>, String> = Map::new("test/pagination");
        let total_num: u64 = 234;
        for i in 1..total_num + 1 {
            let _ = test_state.save(
                deps.as_mut().storage,
                i.to_be_bytes().to_vec(),
                &"test".to_string(),
            );
        }

        test_total_count(deps.as_mut().storage, test_state.clone(), total_num);
        test_offset_and_limit(deps.as_mut().storage, test_state.clone());
        test_ascending_pagination(deps.as_mut().storage, test_state.clone(), total_num);
        test_descending_pagination(deps.as_mut().storage, test_state.clone(), total_num);
    }

    fn test_total_count(storage: &dyn Storage, test_state: Map<Vec<u8>, String>, total_num: u64) {
        let mut key: u64 = 0;
        let res = paginate(
            storage,
            test_state.clone(),
            Some(PageRequest {
                key: vec![],
                offset: 0,
                limit: 0,
                count_total: true,
                reverse: false,
            }),
            |x| -> Option<StdError> {
                key += 1;
                assert_eq!(
                    key,
                    u64::from_be_bytes(x.0.as_slice()[0..8].try_into().unwrap())
                );
                None
            },
        )
        .unwrap();
        assert_eq!(total_num, res.total);
    }

    fn test_offset_and_limit(storage: &dyn Storage, test_state: Map<Vec<u8>, String>) {
        let offset = 30;
        let limit = 40;
        let mut key: u64 = offset;
        let _ = paginate(
            storage,
            test_state.clone(),
            Some(PageRequest {
                key: vec![],
                offset: offset,
                limit: limit,
                count_total: true,
                reverse: false,
            }),
            |x| -> Option<StdError> {
                key += 1;
                assert_eq!(
                    key,
                    u64::from_be_bytes(x.0.as_slice()[0..8].try_into().unwrap())
                );
                None
            },
        )
        .unwrap();
        assert_eq!(offset + limit, key);
    }

    fn test_ascending_pagination(
        storage: &dyn Storage,
        test_state: Map<Vec<u8>, String>,
        total_num: u64,
    ) {
        let mut key: u64 = 0;
        let mut res = PageResponse::default();
        for _ in 1..4 {
            res = paginate(
                storage,
                test_state.clone(),
                Some(PageRequest {
                    key: res.next_key,
                    offset: 0,
                    limit: 0,
                    count_total: true,
                    reverse: false,
                }),
                |x| -> Option<StdError> {
                    key += 1;
                    assert_eq!(
                        key,
                        u64::from_be_bytes(x.0.as_slice()[0..8].try_into().unwrap())
                    );
                    None
                },
            )
            .unwrap();
        }
        assert_eq!(total_num, key);
    }

    fn test_descending_pagination(
        storage: &dyn Storage,
        test_state: Map<Vec<u8>, String>,
        total_num: u64,
    ) {
        let mut key: u64 = total_num + 1;
        let mut res = PageResponse::default();
        for _ in 1..4 {
            res = paginate(
                storage,
                test_state.clone(),
                Some(PageRequest {
                    key: res.next_key,
                    offset: 0,
                    limit: 0,
                    count_total: true,
                    reverse: true,
                }),
                |x| -> Option<StdError> {
                    key -= 1;
                    assert_eq!(
                        key,
                        u64::from_be_bytes(x.0.as_slice()[0..8].try_into().unwrap())
                    );
                    None
                },
            )
            .unwrap();
        }
        assert_eq!(1, key);
    }
}
