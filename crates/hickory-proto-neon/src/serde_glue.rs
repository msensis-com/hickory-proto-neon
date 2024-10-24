use hickory_proto::{
  op::{ Edns, Message, MessageParts },
  rr::{ rdata::OPT, DNSClass, Name, RData, Record, RecordType },
};
use serde::{ Deserialize, Serialize };

use crate::rdata::{ MyA, MyAAAA, MyANAME, MyCNAME, MyHTTPS, MyNS, MyPTR };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyMessage {
  header: MyHeader,
  queries: Vec<MyQuery>,
  answers: Vec<MyRecord>,
  name_servers: Vec<MyRecord>,
  additionals: Vec<MyRecord>,
  signature: Vec<MyRecord>,
  edns: Option<MyEdns>,
}

impl MyMessage {
  pub fn serdeify(msg: Message) -> Self {
    Self {
      header: MyHeader::serdeify(msg.header()),
      queries: msg.queries().into_iter().map(MyQuery::serdeify).collect(),
      answers: msg.answers().into_iter().map(MyRecord::serdeify).collect(),
      name_servers: msg.name_servers().into_iter().map(MyRecord::serdeify).collect(),
      additionals: msg.additionals().into_iter().map(MyRecord::serdeify).collect(),
      signature: msg.signature().into_iter().map(MyRecord::serdeify).collect(),
      edns: msg.extensions().clone().map(MyEdns::serdeify),
    }
  }

  pub fn into_proto(self) -> Message {
    let mut msg = MessageParts::default();
    msg.header = self.header.into_proto();

    msg.queries = self.queries.into_iter().map(MyQuery::into_proto).collect();
    msg.answers = self.answers.into_iter().map(MyRecord::into_proto).collect();
    msg.name_servers = self.name_servers.into_iter().map(MyRecord::into_proto).collect();
    msg.additionals = self.additionals.into_iter().map(MyRecord::into_proto).collect();
    msg.sig0 = self.signature.into_iter().map(MyRecord::into_proto).collect();
    msg.edns = self.edns.map(MyEdns::into_proto);

    msg.into()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyEdns {
  // high 8 bits that make up the 12 bit total field when included with the 4bit rcode from the
  //  header (from TTL)
  rcode_high: u8,
  // Indicates the implementation level of the setter. (from TTL)
  version: u8,
  // Is DNSSEC supported (from TTL)
  dnssec_ok: bool,
  // max payload size, minimum of 512, (from RR CLASS)
  max_payload: u16,

  options: OPT,
}
impl MyEdns {
  pub fn serdeify(edns: Edns) -> Self {
    Self {
      rcode_high: edns.rcode_high(),
      version: edns.version(),
      dnssec_ok: edns.dnssec_ok(),
      max_payload: edns.max_payload(),
      options: edns.options().clone(),
    }
  }

  pub fn into_proto(self) -> Edns {
    let mut edns = Edns::default();
    edns.set_rcode_high(self.rcode_high);
    edns.set_version(self.version);
    edns.set_dnssec_ok(self.dnssec_ok);
    edns.set_max_payload(self.max_payload);

    let options = edns.options_mut();
    for (_, value) in self.options.as_ref().iter() {
      options.insert(value.clone());
    }

    edns
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyHeader {
  id: u16,
  message_type: MyMessageType,
  op_code: MyOpCode,
  authoritative: bool,
  truncation: bool,
  recursion_desired: bool,
  recursion_available: bool,
  authentic_data: bool,
  checking_disabled: bool,
  response_code: MyResponseCode,
  query_count: u16,
  answer_count: u16,
  name_server_count: u16,
  additional_count: u16,
}
impl MyHeader {
  pub fn serdeify(header: &hickory_proto::op::Header) -> Self {
    Self {
      id: header.id(),
      message_type: MyMessageType::serdeify(header.message_type()),
      op_code: MyOpCode::serdeify(header.op_code()),
      authoritative: header.authoritative(),
      truncation: header.truncated(),
      recursion_desired: header.recursion_desired(),
      recursion_available: header.recursion_available(),
      authentic_data: header.authentic_data(),
      checking_disabled: header.checking_disabled(),
      response_code: MyResponseCode::serdeify(header.response_code()),
      query_count: header.query_count(),
      answer_count: header.answer_count(),
      name_server_count: header.name_server_count(),
      additional_count: header.additional_count(),
    }
  }

  pub fn into_proto(self) -> hickory_proto::op::Header {
    let mut header = hickory_proto::op::Header::default();
    header.set_id(self.id);
    header.set_message_type(match self.message_type {
      MyMessageType::Query => hickory_proto::op::MessageType::Query,
      MyMessageType::Response => hickory_proto::op::MessageType::Response,
    });
    header.set_op_code(match self.op_code {
      MyOpCode::Query => hickory_proto::op::OpCode::Query,
      MyOpCode::Status => hickory_proto::op::OpCode::Status,
      MyOpCode::Notify => hickory_proto::op::OpCode::Notify,
      MyOpCode::Update => hickory_proto::op::OpCode::Update,
    });
    header.set_authoritative(self.authoritative);
    header.set_truncated(self.truncation);
    header.set_recursion_desired(self.recursion_desired);
    header.set_recursion_available(self.recursion_available);
    header.set_authentic_data(self.authentic_data);
    header.set_checking_disabled(self.checking_disabled);
    header.set_response_code(self.response_code.into_proto());
    header.set_query_count(self.query_count);
    header.set_answer_count(self.answer_count);
    header.set_name_server_count(self.name_server_count);
    header.set_additional_count(self.additional_count);

    header
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyQuery {
  name: Name,
  query_type: RecordType,
  query_class: DNSClass,
}
impl MyQuery {
  pub fn serdeify(query: &hickory_proto::op::Query) -> Self {
    Self {
      name: query.name().clone(),
      query_type: query.query_type(),
      query_class: query.query_class(),
    }
  }

  pub fn into_proto(self) -> hickory_proto::op::Query {
    let mut query = hickory_proto::op::Query::default();
    query.set_name(self.name);
    query.set_query_type(self.query_type);
    query.set_query_class(self.query_class);
    query
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MyMessageType {
  /// Queries are Client requests, these are either Queries or Updates
  Query,
  /// Response message from the Server or upstream Resolver
  Response,
}
impl MyMessageType {
  pub fn serdeify(msg_type: hickory_proto::op::MessageType) -> Self {
    match msg_type {
      hickory_proto::op::MessageType::Query => MyMessageType::Query,
      hickory_proto::op::MessageType::Response => MyMessageType::Response,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum MyOpCode {
  /// Query request [RFC 1035](https://tools.ietf.org/html/rfc1035)
  Query,

  /// Status message [RFC 1035](https://tools.ietf.org/html/rfc1035)
  Status,

  /// Notify of change [RFC 1996](https://tools.ietf.org/html/rfc1996)
  Notify,

  /// Update message [RFC 2136](https://tools.ietf.org/html/rfc2136)
  Update,
}
impl MyOpCode {
  pub fn serdeify(op_code: hickory_proto::op::OpCode) -> Self {
    match op_code {
      hickory_proto::op::OpCode::Query => MyOpCode::Query,
      hickory_proto::op::OpCode::Status => MyOpCode::Status,
      hickory_proto::op::OpCode::Notify => MyOpCode::Notify,
      hickory_proto::op::OpCode::Update => MyOpCode::Update,
    }
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MyResponseCode {
  NoError,
  FormErr,
  ServFail,
  NXDomain,
  NotImp,
  Refused,
  YXDomain,
  YXRRSet,
  NXRRSet,
  NotAuth,
  NotZone,
  BADVERS,
  BADSIG,
  BADKEY,
  BADTIME,
  BADMODE,
  BADNAME,
  BADALG,
  BADTRUNC,
  BADCOOKIE,
  Unknown(u16),
}
impl MyResponseCode {
  pub fn serdeify(response_code: hickory_proto::op::ResponseCode) -> Self {
    match response_code {
      hickory_proto::op::ResponseCode::NoError => MyResponseCode::NoError,
      hickory_proto::op::ResponseCode::FormErr => MyResponseCode::FormErr,
      hickory_proto::op::ResponseCode::ServFail => MyResponseCode::ServFail,
      hickory_proto::op::ResponseCode::NXDomain => MyResponseCode::NXDomain,
      hickory_proto::op::ResponseCode::NotImp => MyResponseCode::NotImp,
      hickory_proto::op::ResponseCode::Refused => MyResponseCode::Refused,
      hickory_proto::op::ResponseCode::YXDomain => MyResponseCode::YXDomain,
      hickory_proto::op::ResponseCode::YXRRSet => MyResponseCode::YXRRSet,
      hickory_proto::op::ResponseCode::NXRRSet => MyResponseCode::NXRRSet,
      hickory_proto::op::ResponseCode::NotAuth => MyResponseCode::NotAuth,
      hickory_proto::op::ResponseCode::NotZone => MyResponseCode::NotZone,
      hickory_proto::op::ResponseCode::BADVERS => MyResponseCode::BADVERS,
      hickory_proto::op::ResponseCode::BADSIG => MyResponseCode::BADSIG,
      hickory_proto::op::ResponseCode::BADKEY => MyResponseCode::BADKEY,
      hickory_proto::op::ResponseCode::BADTIME => MyResponseCode::BADTIME,
      hickory_proto::op::ResponseCode::BADMODE => MyResponseCode::BADMODE,
      hickory_proto::op::ResponseCode::BADNAME => MyResponseCode::BADNAME,
      hickory_proto::op::ResponseCode::BADALG => MyResponseCode::BADALG,
      hickory_proto::op::ResponseCode::BADTRUNC => MyResponseCode::BADTRUNC,
      hickory_proto::op::ResponseCode::BADCOOKIE => MyResponseCode::BADCOOKIE,
      hickory_proto::op::ResponseCode::Unknown(code) => MyResponseCode::Unknown(code),
    }
  }

  pub fn into_proto(self) -> hickory_proto::op::ResponseCode {
    match self {
      MyResponseCode::NoError => hickory_proto::op::ResponseCode::NoError,
      MyResponseCode::FormErr => hickory_proto::op::ResponseCode::FormErr,
      MyResponseCode::ServFail => hickory_proto::op::ResponseCode::ServFail,
      MyResponseCode::NXDomain => hickory_proto::op::ResponseCode::NXDomain,
      MyResponseCode::NotImp => hickory_proto::op::ResponseCode::NotImp,
      MyResponseCode::Refused => hickory_proto::op::ResponseCode::Refused,
      MyResponseCode::YXDomain => hickory_proto::op::ResponseCode::YXDomain,
      MyResponseCode::YXRRSet => hickory_proto::op::ResponseCode::YXRRSet,
      MyResponseCode::NXRRSet => hickory_proto::op::ResponseCode::NXRRSet,
      MyResponseCode::NotAuth => hickory_proto::op::ResponseCode::NotAuth,
      MyResponseCode::NotZone => hickory_proto::op::ResponseCode::NotZone,
      MyResponseCode::BADVERS => hickory_proto::op::ResponseCode::BADVERS,
      MyResponseCode::BADSIG => hickory_proto::op::ResponseCode::BADSIG,
      MyResponseCode::BADKEY => hickory_proto::op::ResponseCode::BADKEY,
      MyResponseCode::BADTIME => hickory_proto::op::ResponseCode::BADTIME,
      MyResponseCode::BADMODE => hickory_proto::op::ResponseCode::BADMODE,
      MyResponseCode::BADNAME => hickory_proto::op::ResponseCode::BADNAME,
      MyResponseCode::BADALG => hickory_proto::op::ResponseCode::BADALG,
      MyResponseCode::BADTRUNC => hickory_proto::op::ResponseCode::BADTRUNC,
      MyResponseCode::BADCOOKIE => hickory_proto::op::ResponseCode::BADCOOKIE,
      MyResponseCode::Unknown(code) => hickory_proto::op::ResponseCode::Unknown(code),
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
enum MyRData {
  A(crate::rdata::MyA),
  AAAA(crate::rdata::MyAAAA),
  ANAME(crate::rdata::MyANAME),
  CAA(hickory_proto::rr::rdata::CAA),
  CNAME(crate::rdata::MyCNAME),
  CSYNC(hickory_proto::rr::rdata::CSYNC),
  HINFO(hickory_proto::rr::rdata::HINFO),
  HTTPS(crate::rdata::MyHTTPS),
  MX(hickory_proto::rr::rdata::MX),
  NAPTR(hickory_proto::rr::rdata::NAPTR),
  NULL(hickory_proto::rr::rdata::NULL),
  NS(crate::rdata::MyNS),
  OPENPGPKEY(hickory_proto::rr::rdata::OPENPGPKEY),
  OPT(hickory_proto::rr::rdata::OPT),
  PTR(crate::rdata::MyPTR),
  SOA(hickory_proto::rr::rdata::SOA),
  SRV(hickory_proto::rr::rdata::SRV),
  SSHFP(hickory_proto::rr::rdata::SSHFP),
  SVCB(hickory_proto::rr::rdata::SVCB),
  TLSA(hickory_proto::rr::rdata::TLSA),
  TXT(hickory_proto::rr::rdata::TXT),

  /// Unknown RecordData is for record types not supported by Hickory DNS
  Unknown {
    /// RecordType code
    code: RecordType,
    /// RData associated to the record
    rdata: hickory_proto::rr::rdata::NULL,
  },

  ZERO,
}

impl MyRData {
  pub fn serdeify(data: &RData) -> MyRData {
    match data {
      RData::A(a) => MyRData::A(MyA::serdeify(a.clone())),
      RData::AAAA(aaaa) => MyRData::AAAA(MyAAAA::serdeify(aaaa.clone())),
      RData::ANAME(aname) => MyRData::ANAME(MyANAME::serdeify(aname.clone())),
      RData::CAA(caa) => MyRData::CAA(caa.clone()),
      RData::CNAME(cname) => MyRData::CNAME(MyCNAME::serdeify(cname.clone())),
      RData::CSYNC(csync) => MyRData::CSYNC(csync.clone()),
      RData::HINFO(hinfo) => MyRData::HINFO(hinfo.clone()),
      RData::HTTPS(https) => MyRData::HTTPS(MyHTTPS::serdeify(https.clone())),
      RData::MX(mx) => MyRData::MX(mx.clone()),
      RData::NAPTR(naptr) => MyRData::NAPTR(naptr.clone()),
      RData::NULL(null) => MyRData::NULL(null.clone()),
      RData::NS(ns) => MyRData::NS(MyNS::serdeify(ns.clone())),
      RData::OPENPGPKEY(openpgpkey) => MyRData::OPENPGPKEY(openpgpkey.clone()),
      RData::OPT(opt) => MyRData::OPT(opt.clone()),
      RData::PTR(ptr) => MyRData::PTR(MyPTR::serdeify(ptr.clone())),
      RData::SOA(soa) => MyRData::SOA(soa.clone()),
      RData::SRV(srv) => MyRData::SRV(srv.clone()),
      RData::SSHFP(sshfp) => MyRData::SSHFP(sshfp.clone()),
      RData::SVCB(svcb) => MyRData::SVCB(svcb.clone()),
      RData::TLSA(tlsa) => MyRData::TLSA(tlsa.clone()),
      RData::TXT(txt) => MyRData::TXT(txt.clone()),
      RData::Unknown { code, rdata } =>
        MyRData::Unknown { code: code.clone(), rdata: rdata.clone() },
      _ => unreachable!(),
    }
  }

  pub fn into_proto(self) -> RData {
    match self {
      Self::A(a) => RData::A(MyA::into_proto(a)),
      Self::AAAA(aaaa) => RData::AAAA(MyAAAA::into_proto(aaaa)),
      Self::ANAME(aname) => RData::ANAME(MyANAME::into_proto(aname)),
      Self::CAA(caa) => RData::CAA(caa.clone()),
      Self::CNAME(cname) => RData::CNAME(MyCNAME::into_proto(cname)),
      Self::CSYNC(csync) => RData::CSYNC(csync.clone()),
      Self::HINFO(hinfo) => RData::HINFO(hinfo.clone()),
      Self::HTTPS(https) => RData::HTTPS(MyHTTPS::into_proto(https)),
      Self::MX(mx) => RData::MX(mx.clone()),
      Self::NAPTR(naptr) => RData::NAPTR(naptr.clone()),
      Self::NULL(null) => RData::NULL(null.clone()),
      Self::NS(ns) => RData::NS(MyNS::into_proto(ns)),
      Self::OPENPGPKEY(openpgpkey) => RData::OPENPGPKEY(openpgpkey.clone()),
      Self::OPT(opt) => RData::OPT(opt.clone()),
      Self::PTR(ptr) => RData::PTR(MyPTR::into_proto(ptr)),
      Self::SOA(soa) => RData::SOA(soa.clone()),
      Self::SRV(srv) => RData::SRV(srv.clone()),
      Self::SSHFP(sshfp) => RData::SSHFP(sshfp.clone()),
      Self::SVCB(svcb) => RData::SVCB(svcb.clone()),
      Self::TLSA(tlsa) => RData::TLSA(tlsa.clone()),
      Self::TXT(txt) => RData::TXT(txt.clone()),
      Self::Unknown { code, rdata } => RData::Unknown { code: code.clone(), rdata: rdata.clone() },
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyRecord {
  name: Name,
  rr_type: RecordType,
  dns_class: DNSClass,
  ttl: u32,
  rdata: Option<MyRData>,
}

impl MyRecord {
  pub fn serdeify(query: &Record) -> Self {
    Self {
      dns_class: query.dns_class(),
      name: query.name().clone(),
      rr_type: query.record_type(),
      ttl: query.ttl(),
      rdata: query.data().map(MyRData::serdeify),
    }
  }

  pub fn into_proto(self) -> Record {
    let mut query = Record::default();
    query.set_name(self.name);
    query.set_rr_type(self.rr_type);
    query.set_dns_class(self.dns_class);
    query.set_ttl(self.ttl);
    query.set_data(self.rdata.map(MyRData::into_proto));
    query
  }
}
