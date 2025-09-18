use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread::sleep;
use std::error::Error;

use bc_exch_api_funcs::bybit::market::symbols::symbols_a;
use rustc_hash::FxHashMap;

use crate::constt::S;
use crate::other_api::send_to_other_api;


pub async fn symbols() -> FxHashMap<String, f64> 
{
    symbols_a(
        &S.setdef.api_url,
        &S.setdef.category,
        "",
        "",
        "",
    )
        .await
        .into_iter()
        .filter(|v| {
            !S.setdef.black_list_symbols.contains(&v.symbol) 
            && !S.setdef.black_list_coins.iter().any(|v1| v.symbol.contains(v1))
            && {if S.setdef.symbols.len() == 0 {true}else {S.setdef.symbols.contains(&v.symbol)}}
        })
        .map(|s| (s.symbol, s.lastPrice.parse().unwrap()))
        .collect()
}

pub async fn updt(
    mut lasttime: u64,
    mut oldtime: u64,
    mut lastprice: FxHashMap<String, f64>,
    mut oldprice: FxHashMap<String, f64>,
) -> Result<(
    u64, 
    u64, 
    FxHashMap<String, f64>,
    FxHashMap<String, f64>,
), Box<dyn Error>>
{
    if lasttime - oldtime >= S.setdef.update {
        oldprice = lastprice;
        oldtime = lasttime; 
    }
    lasttime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    lastprice = symbols().await;
    Ok((lasttime, oldtime, lastprice, oldprice))
}

pub async fn scrnr() -> Result<(), Box<dyn Error>>
{
    let mut lastprice = symbols().await;
    let mut oldprice = symbols().await;
    let mut lasttime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let mut oldtime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    loop {
        (lasttime, oldtime, lastprice, oldprice) = updt(lasttime, oldtime, lastprice, oldprice).await?;
        for ((oldsmbl, oldprc), (newsmbl, newprc)) in oldprice
            .iter_mut()
            .zip(lastprice.iter_mut())
        {
            let div = (*newprc - *oldprc) / *oldprc;
            let divabs = div.abs();
            if {
                oldsmbl == newsmbl
                && divabs >= S.setdef.th
                && divabs <= S.setdef.limit
                && !S.setdef.black_list_symbols.contains(newsmbl)
                && !S.setdef.black_list_coins.iter().any(|v| newsmbl.contains(v))
                && {S.setdef.symbols.len() == 0 || S.setdef.symbols.contains(newsmbl)}
            } {
                let msg = format!("{newsmbl}: {:.2}%", div * 100.0);
                println!("{}", &msg);
                *oldprc = *newprc;
                send_to_other_api(&msg, S.other_api.as_slice()).await?;
            }
        }
        sleep(Duration::from_secs(S.setdef.delay_req_sec));
        println!("last time update: {lasttime}");
    }
}
