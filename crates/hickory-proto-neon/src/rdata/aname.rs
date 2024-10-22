use hickory_proto::rr::Name;
use hickory_proto::rr::rdata::ANAME;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MyANAME(pub Name);

impl MyANAME {
  pub fn serdeify(aname: ANAME) -> Self {
    Self(aname.0)
  }

  pub fn into_proto(a: Self) -> ANAME {
    ANAME(a.0)
  }
}
