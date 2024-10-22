use hickory_proto::rr::{ rdata::PTR, Name };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MyPTR(pub Name);

impl MyPTR {
  pub fn serdeify(ptr: PTR) -> Self {
    Self(ptr.0)
  }

  pub fn into_proto(a: Self) -> PTR {
    PTR(a.0)
  }
}
