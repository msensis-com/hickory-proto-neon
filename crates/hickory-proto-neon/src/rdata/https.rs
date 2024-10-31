use hickory_proto::rr::rdata::HTTPS;
use serde::{ Deserialize, Serialize };

use super::MySVCB;

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MyHTTPS(pub super::MySVCB);

impl MyHTTPS {
  pub fn serdeify(https: HTTPS) -> Self {
    Self(MySVCB::serdeify(https.0))
  }

  pub fn into_proto(a: Self) -> HTTPS {
    HTTPS(MySVCB::into_proto(a.0))
  }
}
