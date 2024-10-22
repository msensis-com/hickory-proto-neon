use hickory_proto::rr::rdata::A;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MyA(pub std::net::Ipv4Addr);

impl MyA {
  pub fn serdeify(a: A) -> Self {
    Self(a.0)
  }

  pub fn into_proto(a: Self) -> A {
    A(a.0)
  }
}
