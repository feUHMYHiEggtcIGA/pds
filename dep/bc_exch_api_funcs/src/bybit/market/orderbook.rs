#![allow(non_camel_case_types)]

use reqwest::{
    Error as Error_req,
    get,
};
use futures::future::join_all;
use bc_utils_lg::structs::exch::bybit::result::RESULT_EXCH_BYBIT;
use bc_utils_lg::structs::exch::bybit::orderbook::RESULT_ORDERBOOK;
use bc_utils_lg::types::maps::MAP;
use bc_utils_core::mechanisms::all_or_nothing;

use crate::bybit::const_url::ORDERBOOK;

pub async fn orderbook_req(
    api_url: &str,
    category: &str,
    symbol: &str,
    limit: &usize,
) -> Result<RESULT_EXCH_BYBIT<RESULT_ORDERBOOK>, Error_req>
{
    get(format!(
        "{api_url}\
        {ORDERBOOK}\
        ?category={category}\
        &symbol={symbol}\
        &limit={limit}"
    ))
        .await?
        .json()
        .await
}

pub async fn orderbook(
    api_url: &str,
    category: &str,
    symbol: &str,
    limit: &usize,
) -> Result<RESULT_ORDERBOOK, Box<dyn std::error::Error>>
{
    Ok(orderbook_req(api_url, category, symbol, limit).await?.result)
}

pub async fn orderbook_a(
    api_url: &str,
    category: &str,
    symbol: &str,
    limit: &usize,
) -> RESULT_ORDERBOOK
{
    all_or_nothing(async || orderbook(
        api_url, 
        category, 
        symbol, 
        limit
    ).await).await
}

pub async fn orderbooks<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    limit: &usize,
) -> MAP<&'a str, Result<RESULT_ORDERBOOK, Box<dyn std::error::Error>>>
{
    join_all(
        symbols
            .iter()
            .map(
                |v| async {
                    (
                        v.as_str(), 
                        orderbook(
                            api_url, 
                            category, 
                            v.as_str(), 
                            limit,
                        ).await
                    )
                }
            )
    )
        .await
        .into_iter()
        .collect()
}

pub async fn orderbooks_a<'a>(
    api_url: &str,
    category: &str,
    symbols: &'a [String],
    limit: &usize,
) -> MAP<&'a str, RESULT_ORDERBOOK>
{
    join_all(
        symbols
            .iter()
            .map(
                |v| async {
                    (
                        v.as_str(), 
                        orderbook_a(
                            api_url, 
                            category, 
                            v.as_str(), 
                            limit,
                        ).await
                    )
                }
            )
    )
        .await
        .into_iter()
        .collect()
}
