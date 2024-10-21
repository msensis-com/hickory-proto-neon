import { Message } from "./types";

declare module "@msensis/hickory-proto-neon" {
  export function encodePacket(message: Message): Uint8Array;
  export function decodePacket(bytes: Uint8Array): Message;
  export function createAnswer(message?: Message): Message;
}
