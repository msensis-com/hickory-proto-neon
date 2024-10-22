use hickory_proto::rr::{ rdata::NS, Name };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MyNS(pub Name);

impl MyNS {
  pub fn serdeify(ns: NS) -> Self {
    Self(ns.0)
  }

  pub fn into_proto(a: Self) -> NS {
    NS(a.0)
  }
}
