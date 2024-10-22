mod serde_glue;
mod rdata;

use serde_glue::{ MyMessage, MyRecord };
use neon::{ prelude::*, types::buffer::TypedArray };
use hickory_proto::{
  op::Message,
  rr::Record,
  serialize::binary::{ BinDecodable, BinDecoder, BinEncodable, BinEncoder },
};

fn encode(mut cx: FunctionContext) -> JsResult<JsUint8Array> {
  let packet_obj = cx.argument::<JsValue>(0)?;
  let packet: serde_glue::MyMessage = match neon_serde4::from_value(&mut cx, packet_obj) {
    Ok(value) => value,
    Err(e) => {
      return cx.throw_error(e.to_string());
    }
  };

  let mut buffer: Vec<u8> = vec![];
  let mut encoder = BinEncoder::new(&mut buffer);
  MyMessage::into_proto(packet)
    .emit(&mut encoder)
    .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?;

  let uint8array = JsUint8Array::new(&mut cx, buffer.len())?;
  let mut uint8arraybuffer = uint8array.buffer(&mut cx);
  let uint8arraybuffer_slice = uint8arraybuffer.as_mut_slice(&mut cx);
  for (idx, byte) in buffer.iter().enumerate() {
    uint8arraybuffer_slice[idx] = *byte;
  }

  Ok(uint8array)
}

fn decode(mut cx: FunctionContext) -> JsResult<JsValue> {
  let buffer = cx.argument::<JsUint8Array>(0)?.as_slice(&cx);

  let mut decoder = BinDecoder::new(buffer);
  let Ok(message) = Message::read(&mut decoder) else {
    cx.throw_error(&format!("Failed to decode message: {:?}", buffer))?;
    unreachable!();
  };

  Ok(
    neon_serde4
      ::to_value(&mut cx, &MyMessage::serdeify(message))
      .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?
  )
}

fn create_answer(mut cx: FunctionContext) -> JsResult<JsValue> {
  let mut packet: Message;
  let packet_obj = cx.argument_opt(0);
  if let Some(request) = packet_obj {
    match neon_serde4::from_value::<_, MyMessage>(&mut cx, request) {
      Ok(value) => {
        packet = value.into_proto();
      }
      Err(e) => {
        return cx.throw_error(e.to_string());
      }
    };
  } else {
    packet = Message::new();
  }

  packet.set_message_type(hickory_proto::op::MessageType::Response);
  packet.truncate();

  let message = MyMessage::serdeify(packet);
  Ok(
    neon_serde4
      ::to_value(&mut cx, &message)
      .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?
  )
}

fn encode_record(mut cx: FunctionContext) -> JsResult<JsUint8Array> {
  let packet_obj = cx.argument::<JsValue>(0)?;
  let packet: MyRecord = match neon_serde4::from_value(&mut cx, packet_obj) {
    Ok(value) => value,
    Err(e) => {
      return cx.throw_error(e.to_string());
    }
  };

  let mut buffer: Vec<u8> = vec![];
  let mut encoder = BinEncoder::new(&mut buffer);
  packet
    .into_proto()
    .emit(&mut encoder)
    .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?;

  let uint8array = JsUint8Array::new(&mut cx, buffer.len())?;
  let mut uint8arraybuffer = uint8array.buffer(&mut cx);
  let uint8arraybuffer_slice = uint8arraybuffer.as_mut_slice(&mut cx);
  for (idx, byte) in buffer.iter().enumerate() {
    uint8arraybuffer_slice[idx] = *byte;
  }

  Ok(uint8array)
}

fn decode_record(mut cx: FunctionContext) -> JsResult<JsValue> {
  let buffer = cx.argument::<JsUint8Array>(0)?.as_slice(&cx);

  let mut decoder = BinDecoder::new(buffer);
  let Ok(record) = Record::read(&mut decoder) else {
    cx.throw_error(&format!("Failed to decode message: {:?}", buffer))?;
    unreachable!();
  };

  Ok(
    neon_serde4
      ::to_value(&mut cx, &MyRecord::serdeify(&record))
      .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?
  )
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("encodePacket", encode)?;
  cx.export_function("decodePacket", decode)?;
  cx.export_function("createAnswer", create_answer)?;
  cx.export_function("encode_record", encode_record)?;
  cx.export_function("decode_record", decode_record)?;
  Ok(())
}
