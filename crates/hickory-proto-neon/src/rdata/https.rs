use hickory_proto::rr::rdata::{ HTTPS, SVCB };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MyHTTPS(pub SVCB);

impl MyHTTPS {
  pub fn serdeify(https: HTTPS) -> Self {
    Self(https.0)
  }

  pub fn into_proto(a: Self) -> HTTPS {
    HTTPS(a.0)
  }
}
