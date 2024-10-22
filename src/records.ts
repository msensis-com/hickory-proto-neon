import { RecordType } from "./types";

type Address = string;
type Name = string;

export namespace Records {
  export type A = Address;
  export type AAAA = Address;
  export type CAA = unknown;
  export type ANAME = Name;
  export type CNAME = Name;
  export type CSYNC = unknown;
  export type HINFO = unknown;
  export type HTTPS = unknown;
  export type MX = unknown;
  export type NAPTR = unknown;
  export type NULL = unknown;
  export type NS = string;
  export type OPENPGPKEY = { public_key: Uint8Array };
  export type OPT = unknown;
  export type PTR = Address;
  export type SOA = unknown;

  export type SRV = {
    priority: number;
    weight: number;
    port: number;
    target: Name;
  };

  export type SSHFP = unknown;
  export type SVCB = unknown;
  export type TLSA = unknown;
  export type TXT = { txt_data: Uint8Array };
  export type UNKNOWN = { code: RecordType; rdata: NULL };
}
