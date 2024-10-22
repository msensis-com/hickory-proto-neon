use hickory_proto::rr::rdata::AAAA;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MyAAAA(pub std::net::Ipv6Addr);

impl MyAAAA {
  pub fn serdeify(aaaa: AAAA) -> Self {
    Self(aaaa.0)
  }

  pub fn into_proto(a: Self) -> AAAA {
    AAAA(a.0)
  }
}
