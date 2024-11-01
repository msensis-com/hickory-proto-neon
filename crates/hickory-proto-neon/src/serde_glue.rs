use hickory_proto::{
  op::{ Edns, Header, Message, MessageParts, Query },
  rr::{ rdata::OPT, DNSClass, Name, RData, Record, RecordType },
};
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyMessage {
  header: Header,
  queries: Vec<Query>,
  answers: Vec<MyRecord>,
  name_servers: Vec<MyRecord>,
  additionals: Vec<MyRecord>,
  signature: Vec<MyRecord>,
  edns: Option<Edns>,
}

impl MyMessage {
  pub fn serdeify(msg: Message) -> Self {
    Self {
      header: msg.header().clone(),
      queries: msg.queries().to_vec(),
      answers: msg.answers().into_iter().map(MyRecord::serdeify).collect(),
      name_servers: msg.name_servers().into_iter().map(MyRecord::serdeify).collect(),
      additionals: msg.additionals().into_iter().map(MyRecord::serdeify).collect(),
      signature: msg.signature().into_iter().map(MyRecord::serdeify).collect(),
      edns: msg.extensions().clone(),
    }
  }

  pub fn into_proto(self) -> Message {
    let mut msg = MessageParts::default();
    msg.header = self.header.clone();

    msg.queries = self.queries;
    msg.answers = self.answers.into_iter().map(MyRecord::into_proto).collect();
    msg.name_servers = self.name_servers.into_iter().map(MyRecord::into_proto).collect();
    msg.additionals = self.additionals.into_iter().map(MyRecord::into_proto).collect();
    msg.sig0 = self.signature.into_iter().map(MyRecord::into_proto).collect();
    msg.edns = self.edns;

    msg.into()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyRecord {
  name: Name,
  rr_type: RecordType,
  dns_class: DNSClass,
  ttl: u32,
  rdata: RData,
}

impl MyRecord {
  pub fn serdeify(query: &Record) -> Self {
    Self {
      dns_class: query.dns_class(),
      name: query.name().clone(),
      rr_type: query.record_type(),
      ttl: query.ttl(),
      rdata: query.data().clone(),
    }
  }

  pub fn into_proto(self) -> Record {
    let mut query = Record::from_rdata(self.name, self.ttl, self.rdata);
    query.set_dns_class(self.dns_class);
    query
  }
}
