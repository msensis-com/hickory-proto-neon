// This module is the ESM entry point for the library.

import type { Message } from "./types.js";

export * from "./types.js";
export * from "./load.cjs";

declare module "@msensis/hickory-proto-neon" {
  export function encodePacket(message: Message, stream?: boolean): Uint8Array;
  export function decodePacket(bytes: Uint8Array, stream?: boolean): Message;
  export function createAnswer(message?: Message): Message;
}
