use hickory_proto::{
    op::{ Edns, Message, MessageParts },
    rr::{ rdata::OPT, DNSClass, Name, Record, RecordType },
};
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyMessage {
    header: MyHeader,
    queries: Vec<MyQuery>,
    answers: Vec<Record>,
    name_servers: Vec<Record>,
    additionals: Vec<Record>,
    signature: Vec<Record>,
    edns: Option<MyEdns>,
}
impl MyMessage {
    pub fn serdeify(msg: Message) -> Self {
        Self {
            header: MyHeader::serdeify(msg.header()),
            queries: msg.queries().into_iter().map(MyQuery::serdeify).collect(),
            answers: msg
                .answers()
                .iter()
                .map(|x| x.clone())
                .collect(),
            name_servers: msg
                .name_servers()
                .iter()
                .map(|x| x.clone())
                .collect(),
            additionals: msg
                .additionals()
                .iter()
                .map(|x| x.clone())
                .collect(),
            signature: msg
                .signature()
                .iter()
                .map(|x| x.clone())
                .collect(),
            edns: msg.extensions().clone().map(MyEdns::serdeify),
        }
    }

    pub fn into_proto(self) -> Message {
        let mut msg = MessageParts::default();
        msg.header = self.header.into_proto();

        msg.queries = self.queries.into_iter().map(MyQuery::into_proto).collect();
        msg.answers = self.answers;
        msg.name_servers = self.name_servers;
        msg.additionals = self.additionals;
        msg.sig0 = self.signature;
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
