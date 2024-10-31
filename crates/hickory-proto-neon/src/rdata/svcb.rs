use hickory_proto::rr::{
  rdata::{
    svcb::{ Alpn, EchConfig, IpHint, Mandatory, SvcParamKey, SvcParamValue, Unknown },
    A,
    AAAA,
    SVCB,
  },
  Name,
};
use serde::{ Deserialize, Serialize };

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
pub struct MySVCB {
  svc_priority: u16,
  target_name: Name,
  svc_params: Vec<(SvcParamKey, MySvcParamValue)>,
}

impl MySVCB {
  pub fn serdeify(svcb: SVCB) -> Self {
    Self {
      svc_priority: svcb.svc_priority(),
      target_name: svcb.target_name().clone(),
      svc_params: svcb
        .svc_params()
        .into_iter()
        .map(|(key, value)| { (key.clone(), MySvcParamValue::serdeify(value.clone())) })
        .collect(),
    }
  }

  pub fn into_proto(a: Self) -> SVCB {
    SVCB::new(
      a.svc_priority,
      a.target_name,
      a.svc_params
        .into_iter()
        .map(|(key, value)| { (key.clone(), MySvcParamValue::into_proto(value.clone())) })
        .collect()
    )
  }
}

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
pub enum MySvcParamValue {
  Mandatory(MyMandatory),
  Alpn(MyAlpn),
  NoDefaultAlpn,
  Port(u16),
  Ipv4Hint(MyIpHint<A>),
  EchConfig(MyEchConfig),
  Ipv6Hint(MyIpHint<AAAA>),
  Unknown(MyUnknown),
}

impl MySvcParamValue {
  pub fn serdeify(param: SvcParamValue) -> Self {
    match param {
      SvcParamValue::Mandatory(mandatory) => MySvcParamValue::Mandatory(MyMandatory(mandatory.0)),
      SvcParamValue::Alpn(alpn) => MySvcParamValue::Alpn(MyAlpn(alpn.0)),
      SvcParamValue::NoDefaultAlpn => MySvcParamValue::NoDefaultAlpn,
      SvcParamValue::Port(port) => MySvcParamValue::Port(port),
      SvcParamValue::Ipv4Hint(ip_hint) => MySvcParamValue::Ipv4Hint(MyIpHint(ip_hint.0)),
      SvcParamValue::EchConfig(ech_config) => MySvcParamValue::EchConfig(MyEchConfig(ech_config.0)),
      SvcParamValue::Ipv6Hint(ip_hint) => MySvcParamValue::Ipv6Hint(MyIpHint(ip_hint.0)),
      SvcParamValue::Unknown(unknown) => MySvcParamValue::Unknown(MyUnknown(unknown.0)),
    }
  }

  #[rustfmt::skip]
  pub fn into_proto(a: Self) -> SvcParamValue {
    match a {
      MySvcParamValue::Mandatory(my_mandatory) => SvcParamValue::Mandatory(Mandatory(my_mandatory.0)),
      MySvcParamValue::Alpn(my_alpn) => SvcParamValue::Alpn(Alpn(my_alpn.0)),
      MySvcParamValue::NoDefaultAlpn => SvcParamValue::NoDefaultAlpn,
      MySvcParamValue::Port(port) => SvcParamValue::Port(port),
      MySvcParamValue::Ipv4Hint(my_ip_hint) => SvcParamValue::Ipv4Hint(IpHint(my_ip_hint.0)),
      MySvcParamValue::EchConfig(my_ech_config) => SvcParamValue::EchConfig(EchConfig(my_ech_config.0)),
      MySvcParamValue::Ipv6Hint(my_ip_hint) => SvcParamValue::Ipv6Hint(IpHint(my_ip_hint.0)),
      MySvcParamValue::Unknown(my_unknown) => SvcParamValue::Unknown(Unknown(my_unknown.0)),
    }
  }
}

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
struct MyMandatory(pub Vec<SvcParamKey>);

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
struct MyAlpn(pub Vec<String>);

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
struct MyIpHint<T>(pub Vec<T>);

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
struct MyEchConfig(pub Vec<u8>);

#[derive(Debug, Hash, Clone, Deserialize, Serialize)]
#[serde(transparent)]
struct MyUnknown(pub Vec<u8>);
