import type { Message } from "./types.js";

export * from "./types.js";
import load from "./load.cjs";

type Module = {
  encodePacket(message: Message, stream?: boolean): Uint8Array;
  decodePacket(bytes: Uint8Array, stream?: boolean): Message;
  createQuery(message?: Message): Message;
  createResponse(message?: Message): Message;
};

export const { encodePacket, decodePacket, createQuery, createResponse } = load as unknown as Module;
