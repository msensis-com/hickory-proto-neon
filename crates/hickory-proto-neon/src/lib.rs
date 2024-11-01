use neon::{ prelude::*, types::buffer::TypedArray };
use hickory_proto::{
  op::Message,
  serialize::binary::{ BinDecodable, BinDecoder, BinEncodable, BinEncoder },
};

fn encode(mut cx: FunctionContext) -> JsResult<JsUint8Array> {
  let packet_obj = cx.argument::<JsValue>(0)?;
  let packet: Message = match neon_serde4::from_value(&mut cx, packet_obj) {
    Ok(value) => value,
    Err(e) => {
      return cx.throw_error(e.to_string());
    }
  };

  let stream = match cx.argument_opt(1) {
    Some(stream) =>
      neon_serde4
        ::from_value::<_, bool>(&mut cx, stream)
        .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?,
    None => false,
  };

  let mut buffer: Vec<u8> = vec![];
  let mut encoder = BinEncoder::new(&mut buffer);
  packet.emit(&mut encoder).map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?;

  if stream {
    // add size prefix
    let mut stream_buffer = Vec::from((buffer.len() as u16).to_be_bytes());
    stream_buffer.append(&mut buffer);
    buffer = stream_buffer;
  }

  let uint8array = JsUint8Array::new(&mut cx, buffer.len())?;
  let mut uint8arraybuffer = uint8array.buffer(&mut cx);
  let uint8arraybuffer_slice = uint8arraybuffer.as_mut_slice(&mut cx);
  for (idx, byte) in buffer.iter().enumerate() {
    uint8arraybuffer_slice[idx] = *byte;
  }

  Ok(uint8array)
}

fn decode(mut cx: FunctionContext) -> JsResult<JsValue> {
  let mut buffer = cx.argument::<JsUint8Array>(0)?.as_slice(&cx).to_vec();
  let stream = match cx.argument_opt(1) {
    Some(stream) =>
      neon_serde4
        ::from_value::<_, bool>(&mut cx, stream)
        .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?,
    None => false,
  };

  if stream {
    // remove size prefix
    buffer.remove(0);
    buffer.remove(0);
  }

  let mut decoder = BinDecoder::new(buffer.as_slice());
  let Ok(message) = Message::read(&mut decoder) else {
    cx.throw_error(&format!("Failed to decode message: {:?}", buffer))?;
    unreachable!();
  };

  Ok(
    neon_serde4
      ::to_value(&mut cx, &message)
      .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?
  )
}

fn create_answer(mut cx: FunctionContext) -> JsResult<JsValue> {
  let mut packet: Message;
  let packet_obj = cx.argument_opt(0);
  if let Some(request) = packet_obj {
    match neon_serde4::from_value::<_, Message>(&mut cx, request) {
      Ok(value) => {
        packet = value;
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

  Ok(
    neon_serde4
      ::to_value(&mut cx, &packet)
      .map_err(|x| cx.throw_error::<_, JsValue>(x.to_string()).unwrap_err())?
  )
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("encodePacket", encode)?;
  cx.export_function("decodePacket", decode)?;
  cx.export_function("createAnswer", create_answer)?;
  Ok(())
}
