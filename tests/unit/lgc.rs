#![warn(unused_assignments)]

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio;

use pds::lgc::{symbols, updt};

#[tokio::test]
async fn symbols_res_1() {
    assert!(symbols().await.unwrap().len() != 0);
}

#[tokio::test]
async fn symbols_res_2() {
    assert!(symbols().await.unwrap().into_iter().all(|v| v.1 != 0.0));
}

// #[tokio::test]
// async fn updt_res_1() -> Result<(), Box<dyn Error>> {
//     let mut lastprcs = symbols().await?;
//     let mut oldprcs = symbols().await?;
//     let oldprcs2 = oldprcs.clone();
//     let mut lasttime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
//     let mut oldtime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
//     (lasttime, oldtime, lastprcs, oldprcs) = updt(lasttime, oldtime, lastprcs, oldprcs).await?;
//     assert_eq!(oldprcs, oldprcs2);
//     Ok(())
// }
