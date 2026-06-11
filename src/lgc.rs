use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bc_constructor::indicators::IndicatorsGateway;
use bc_constructor::map::indicators::{
    FUNCS_EXTRACT_ARGS, get_indicators_from_settings, get_indicators_from_settings_without_bf,
};
use bc_exch_api_funcs::bybit::market::klines::klines_a;
use bc_exch_api_funcs::bybit::market::symbols::symbols_a;
use rustc_hash::FxHashMap;

use crate::constt::S;
use crate::other_api::send_to_other_api;

pub async fn symbols() -> Result<FxHashMap<String, f64>, Box<dyn Error>> {
    Ok(symbols_a(
        &S.setdef.api_url,
        &S.setdef.category,
        "",
        "",
        "",
        S.setdef.wait_ms_req,
        S.setdef.wait_ms_cycle_req,
    )
    .await?
    .into_iter()
    .filter(|v| {
        !S.setdef.black_list_symbols.contains(&v.symbol)
            && !S
                .setdef
                .black_list_coins
                .iter()
                .any(|v1| v.symbol.contains(v1))
            && {
                if S.setdef.symbols.len() == 0 {
                    true
                } else {
                    S.setdef.symbols.contains(&v.symbol)
                }
            }
    })
    .map(|s| (s.symbol, s.lastPrice.parse().unwrap()))
    .collect())
}

pub async fn updt(
    mut lasttime: u64,
    mut oldtime: u64,
    mut lastprice: FxHashMap<String, f64>,
    mut oldprice: FxHashMap<String, f64>,
) -> Result<(u64, u64, FxHashMap<String, f64>, FxHashMap<String, f64>), Box<dyn Error>> {
    if lasttime - oldtime >= S.setdef.update {
        oldprice = lastprice;
        oldtime = lasttime;
    }
    lasttime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    lastprice = symbols().await?;
    Ok((lasttime, oldtime, lastprice, oldprice))
}

pub async fn scrnr() -> Result<(), Box<dyn Error>> {
    let mut lastprice = symbols().await?;
    let mut oldprice = symbols().await?;
    let mut lasttime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let mut oldtime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let funcs_extract_args = FUNCS_EXTRACT_ARGS();
    let indicators_without_bf =
        get_indicators_from_settings_without_bf(&S.setdef.indicators, &funcs_extract_args);
    let w = indicators_without_bf
        .iter()
        .map(|(_, v)| v.w())
        .max()
        .unwrap_or_default();

    loop {
        (lasttime, oldtime, lastprice, oldprice) =
            updt(lasttime, oldtime, lastprice, oldprice).await?;
        for ((oldsmbl, oldprc), (newsmbl, newprc)) in oldprice.iter_mut().zip(lastprice.iter_mut())
        {
            let div = (*newprc - *oldprc) / *oldprc;
            let divabs = div.abs();
            if {
                oldsmbl == newsmbl
                    && divabs >= S.setdef.th
                    && divabs <= S.setdef.limit
                    && !S.setdef.black_list_symbols.contains(newsmbl)
                    && !S
                        .setdef
                        .black_list_coins
                        .iter()
                        .any(|v| newsmbl.contains(v))
                    && { S.setdef.symbols.len() == 0 || S.setdef.symbols.contains(newsmbl) }
            } {
                let mut msg = format!("\n{newsmbl}: {:.2}%", div * 100.0);
                if S.setdef.indicators.len() != 0 {
                    let klines = klines_a(
                        &S.setdef.api_url,
                        &S.setdef.category,
                        newsmbl,
                        &S.setdef.interval,
                        w,
                        0,
                        0,
                        0,
                        0,
                    )
                    .await?;
                    let klines = (0..klines[0].len())
                        .map(|i| klines.iter().map(|v| v[i]).collect::<Vec<f64>>())
                        .collect::<Vec<Vec<f64>>>();
                    let gateway = IndicatorsGateway::new(
                        get_indicators_from_settings(
                            &S.setdef.indicators,
                            &funcs_extract_args,
                            &klines,
                            &indicators_without_bf,
                        ),
                        &indicators_without_bf,
                        &S.setdef.indicators,
                    );
                    for s in gateway
                        .get_indications_from_settings(&klines)
                        .iter()
                        .filter(|v| S.setdef.indicators_used.contains(&v.0.to_string()))
                    {
                        msg.push_str(&format!("\n{}: {:.4}", s.0, s.1));
                    }
                }
                println!("{}", &msg);
                *oldprc = *newprc;
                send_to_other_api(&msg, S.other_api.as_slice()).await?;
            }
        }
        sleep(Duration::from_secs(S.setdef.delay_req_sec));
        println!("last time update: {lasttime}");
    }
}
