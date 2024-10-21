// This module is the ESM entry point for the library.

import type { Message } from "./types.js";
export * from "./index.cjs";

declare module "@msensis/hickory-proto-neon" {
  export function encodePacket(message: Message): Uint8Array;
  export function decodePacket(bytes: Uint8Array): Message;
  export function createAnswer(message?: Message): Message;
}
