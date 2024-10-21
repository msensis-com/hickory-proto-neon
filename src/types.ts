export interface Message {
    header: Header;
    queries: Query[];
    answers: Record[];
    additionals: Record[];
    signature: Record[];
    edns: Edns | null;
}

export interface Header {
    id: number;
    message_type: MessageType;
    op_code: OpCode;
    authoritative: boolean;
    truncation: boolean;
    recursion_desired: boolean;
    recursion_available: boolean;
    authentic_data: boolean;
    checking_disabled: boolean;
    response_code: ResponseCode;
    query_count: number;
    answer_count: number;
    name_server_count: number;
    additional_count: number;
}

export interface Query {
    name: string;
    query_type: RecordType;
    query_class: DNSClass;
}

export interface Record {
    name_labels: string;
    rr_type: RecordType;
    dns_class: DNSClass;
    ttl: number;
    // rdata: Option<R>,
}

export enum RecordType {
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) IPv4 Address record
    A,
    /// [RFC 3596](https://tools.ietf.org/html/rfc3596) IPv6 address record
    AAAA,
    /// [ANAME draft-ietf-dnsop-aname](https://tools.ietf.org/html/draft-ietf-dnsop-aname-04)
    ANAME,
    //  AFSDB,      //	18	RFC 1183	AFS database record
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) All cached records, aka ANY
    ANY,
    //  APL,        //	42	RFC 3123	Address Prefix List
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) Authoritative Zone Transfer
    AXFR,
    /// [RFC 6844](https://tools.ietf.org/html/rfc6844) Certification Authority Authorization
    CAA,
    /// [RFC 7344](https://tools.ietf.org/html/rfc7344) Child DS
    CDS,
    /// [RFC 7344](https://tools.ietf.org/html/rfc7344) Child DNSKEY
    CDNSKEY,
    //  CERT,       // 37 RFC 4398 Certificate record
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) Canonical name record
    CNAME,
    //  DHCID,      // 49 RFC 4701 DHCP identifier
    //  DLV,        //	32769	RFC 4431	DNSSEC Lookaside Validation record
    //  DNAME,      // 39 RFC 2672 Delegation Name
    /// [RFC 7477](https://tools.ietf.org/html/rfc4034) Child-to-parent synchronization record
    CSYNC,
    /// [RFC 4034](https://tools.ietf.org/html/rfc4034) DNS Key record: RSASHA256 and RSASHA512, RFC5702
    DNSKEY,
    /// [RFC 4034](https://tools.ietf.org/html/rfc4034) Delegation signer: RSASHA256 and RSASHA512, RFC5702
    DS,
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) host information
    HINFO,
    //  HIP,        // 55 RFC 5205 Host Identity Protocol
    /// [RFC draft-ietf-dnsop-svcb-https-03](https://tools.ietf.org/html/draft-ietf-dnsop-svcb-httpssvc-03) DNS SVCB and HTTPS RRs
    HTTPS,
    //  IPSECKEY,   // 45 RFC 4025 IPsec Key
    /// [RFC 1996](https://tools.ietf.org/html/rfc1996) Incremental Zone Transfer
    IXFR,
    //  KX,         // 36 RFC 2230 Key eXchanger record
    /// [RFC 2535](https://tools.ietf.org/html/rfc2535) and [RFC 2930](https://tools.ietf.org/html/rfc2930) Key record
    KEY,
    //  LOC,        // 29 RFC 1876 Location record
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) Mail exchange record
    MX,
    /// [RFC 3403](https://tools.ietf.org/html/rfc3403) Naming Authority Pointer
    NAPTR,
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) Name server record
    NS,
    /// [RFC 4034](https://tools.ietf.org/html/rfc4034) Next-Secure record
    NSEC,
    /// [RFC 5155](https://tools.ietf.org/html/rfc5155) NSEC record version 3
    NSEC3,
    /// [RFC 5155](https://tools.ietf.org/html/rfc5155) NSEC3 parameters
    NSEC3PARAM,
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) Null server record, for testing
    NULL,
    /// [RFC 7929](https://tools.ietf.org/html/rfc7929) OpenPGP public key
    OPENPGPKEY,
    /// [RFC 6891](https://tools.ietf.org/html/rfc6891) Option
    OPT,
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) Pointer record
    PTR,
    //  RP,         // 17 RFC 1183 Responsible person
    /// [RFC 4034](https://tools.ietf.org/html/rfc4034) DNSSEC signature: RSASHA256 and RSASHA512, RFC5702
    RRSIG,
    /// [RFC 2535](https://tools.ietf.org/html/rfc2535) (and [RFC 2931](https://tools.ietf.org/html/rfc2931)) Signature, to support [RFC 2137](https://tools.ietf.org/html/rfc2137) Update.
    SIG,
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) and [RFC 2308](https://tools.ietf.org/html/rfc2308) Start of [a zone of] authority record
    SOA,
    /// [RFC 2782](https://tools.ietf.org/html/rfc2782) Service locator
    SRV,
    /// [RFC 4255](https://tools.ietf.org/html/rfc4255) SSH Public Key Fingerprint
    SSHFP,
    /// [RFC draft-ietf-dnsop-svcb-https-03](https://tools.ietf.org/html/draft-ietf-dnsop-svcb-httpssvc-03) DNS SVCB and HTTPS RRs
    SVCB,
    //  TA,         // 32768 N/A DNSSEC Trust Authorities
    //  TKEY,       // 249 RFC 2930 Secret key record
    /// [RFC 6698](https://tools.ietf.org/html/rfc6698) TLSA certificate association
    TLSA,
    /// [RFC 8945](https://tools.ietf.org/html/rfc8945) Transaction Signature
    TSIG,
    /// [RFC 1035](https://tools.ietf.org/html/rfc1035) Text record
    TXT,

    /// Unknown Record type, or unsupported
    // Unknown(u16),

    /// This corresponds to a record type of 0, unspecified
    ZERO,
}

export enum DNSClass {
    /// Internet
    IN,
    /// Chaos
    CH,
    /// Hesiod
    HS,
    /// QCLASS NONE
    NONE,
    /// QCLASS * (ANY)
    ANY,

    // /// Special class for OPT Version, it was overloaded for EDNS - RFC 6891
    // /// From the RFC: `Values lower than 512 MUST be treated as equal to 512`
    // OPT(u16),
    // /// Unknown DNSClass was parsed
    // Unknown(u16),
}

export enum ResponseCode {
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
}

export enum OpCode {
    /// Query request [RFC 1035](https://tools.ietf.org/html/rfc1035)
    Query,

    /// Status message [RFC 1035](https://tools.ietf.org/html/rfc1035)
    Status,

    /// Notify of change [RFC 1996](https://tools.ietf.org/html/rfc1996)
    Notify,

    /// Update message [RFC 2136](https://tools.ietf.org/html/rfc2136)
    Update,
}

export enum MessageType {
    /// Queries are Client requests, these are either Queries or Updates
    Query,
    /// Response message from the Server or upstream Resolver
    Response,
}

export interface Edns {
    rcode_high: number;
    version: number;
    dnssec_ok: boolean;
    max_payload: number;

    options: globalThis.Record<EdnsCode, EdnsOption>
}

export enum EdnsCode {}
export type EdnsOption = {}
