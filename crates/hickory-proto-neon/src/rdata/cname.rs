use hickory_proto::rr::{ rdata::CNAME, Name };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct MyCNAME(pub Name);

impl MyCNAME {
  pub fn serdeify(cname: CNAME) -> Self {
    Self(cname.0)
  }

  pub fn into_proto(a: Self) -> CNAME {
    CNAME(a.0)
  }
}
