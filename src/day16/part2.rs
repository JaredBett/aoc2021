use std::collections::HashMap;

const LITERAL_TYPE_ID: u8 = 4;
const OP_BITS_LENGTH_TYPE_ID: u8 = 0;
const OP_PACKETS_LENGTH_TYPE_ID: u8 = 1;

#[derive(Debug)]
enum Packet {
  Literal(LiteralPacket),
  Operator(OperatorPacket),
}

#[derive(Debug)]
struct Header {
  version: u8,
  type_id: u8,
}

#[derive(Debug)]
struct LiteralPacket {
  header: Header,
  literal: u64,
}

#[derive(Debug)]
struct OperatorPacket {
  header: Header,
  op: Operator,
  sub_packets: Vec<Packet>,
}

#[derive(Debug)]
enum OperatorPacketLength {
  Bits(u16),
  Packets(u16),
}

#[derive(Debug)]
enum Operator {
  Sum,
  Product,
  Min,
  Max,
  Greater,
  Less,
  Equal,
}

pub fn main() {
  // let input = "C200B40A82"; // finds the sum of 1 and 2, resulting in the value 3.
  // let input = "04005AC33890"; // finds the product of 6 and 9, resulting in the value 54.
  // let input = "880086C3E88112"; // finds the minimum of 7, 8, and 9, resulting in the value 7.
  // let input = "CE00C43D881120"; // finds the maximum of 7, 8, and 9, resulting in the value 9.
  // let input = "D8005AC2A8F0"; // produces 1, because 5 is less than 15.
  // let input = "F600BC2D8F"; // produces 0, because 5 is not greater than 15.
  // let input = "9C005AC2F8F0"; // produces 0, because 5 is not equal to 15.
  // let input = "9C0141080250320F1802104A08"; // produces 1, because 1 + 3 = 2 * 2.
  let input = include_str!("input.txt");

  let bin = hex2bin(input);
  // println!("{}", bin);

  let (packet, num_bits) = parse_packet(&bin);
  // dbg!(&packet);
  println!("{}", evaluate(&packet));
}

fn evaluate(packet: &Packet) -> u64 {
  match packet {
    Packet::Literal(lit_packet) => lit_packet.literal,
    Packet::Operator(op_packet) => evaluate_op(op_packet),
  }
}

fn evaluate_op(op_packet: &OperatorPacket) -> u64 {
  let mut values = op_packet.sub_packets.iter().map(evaluate);
  match op_packet.op {
    Operator::Sum => values.sum(),
    Operator::Product => values.fold(1, |prod, x| prod * x),
    Operator::Min => values.min().unwrap(),
    Operator::Max => values.max().unwrap(),
    Operator::Greater => from_bool(values.next().unwrap() > values.next().unwrap()),
    Operator::Less => from_bool(values.next().unwrap() < values.next().unwrap()),
    Operator::Equal => from_bool(values.next().unwrap() == values.next().unwrap()),
  }
}

fn from_bool(b: bool) -> u64 {
  if b {
    1
  } else {
    0
  }
}

fn parse_packet(input: &str) -> (Packet, usize) {
  let mut i = 0;
  // while i < input.len() {
  let (version, num_bits) = parse_version(&input[i..]);
  i += num_bits;

  let (type_id, num_bits) = parse_type_id(&input[i..]);
  i += num_bits;

  let header = Header { version, type_id };

  if type_id == LITERAL_TYPE_ID {
    let (literal, num_bits) = parse_literal(&input[i..]);
    i += num_bits;

    let literal_packet = LiteralPacket { header, literal };

    return (Packet::Literal(literal_packet), i);
  } else {
    // operator packet
    let (sub_packets, num_bits) = parse_op_packet_body(&input[i..]);
    i += num_bits;

    let op = type_id_to_op(header.type_id).unwrap();
    let op_packet = OperatorPacket {
      header,
      op,
      sub_packets,
    };
    return (Packet::Operator(op_packet), i);
  }

  // break;
  // }

  // return (packet, i);
}

fn type_id_to_op(type_id: u8) -> Option<Operator> {
  match type_id {
    0 => Some(Operator::Sum),
    1 => Some(Operator::Product),
    2 => Some(Operator::Min),
    3 => Some(Operator::Max),
    5 => Some(Operator::Greater),
    6 => Some(Operator::Less),
    7 => Some(Operator::Equal),
    _ => None,
  }
}

fn parse_op_packet_body(input: &str) -> (Vec<Packet>, usize) {
  let mut i = 0;

  let length_type_id = u8::from_str_radix(&input[i..i + 1], 2).unwrap();
  i += 1;

  let mut sub_packets: Vec<Packet> = Vec::new();

  let length: OperatorPacketLength;
  if length_type_id == OP_BITS_LENGTH_TYPE_ID {
    let num_bits = u16::from_str_radix(&input[i..i + 15], 2).unwrap();
    i += 15;
    length = OperatorPacketLength::Bits(num_bits);
  } else {
    // if length_type_id == OP_PACKETS_LENGTH_TYPE_ID
    let num_packets = u16::from_str_radix(&input[i..i + 11], 2).unwrap();
    i += 11;
    length = OperatorPacketLength::Packets(num_packets);
  }

  let mut total_op_bits = 0;
  while i < input.len() {
    let (packet, num_bits) = parse_packet(&input[i..]);
    i += num_bits;
    total_op_bits += num_bits;
    sub_packets.push(packet);

    match length {
      OperatorPacketLength::Bits(op_bits) => {
        if total_op_bits >= op_bits as usize {
          break;
        }
      }
      OperatorPacketLength::Packets(num_packets) => {
        if sub_packets.len() >= num_packets as usize {
          break;
        }
      }
    }
  }

  return (sub_packets, i);
}

fn parse_version(input: &str) -> (u8, usize) {
  let num_bits = 3;
  let version = u8::from_str_radix(&input[0..num_bits], 2).unwrap();
  return (version, num_bits);
}

fn parse_type_id(input: &str) -> (u8, usize) {
  let num_bits = 3;
  let version = u8::from_str_radix(&input[0..num_bits], 2).unwrap();
  return (version, num_bits);
}

fn parse_literal(input: &str) -> (u64, usize) {
  let mut literal_bin = String::new();

  let mut more_groups: u8 = 1;
  let mut i = 0;
  while more_groups == 1 {
    more_groups = u8::from_str_radix(&input[i..i + 1], 2).unwrap();
    i += 1;

    let group_bin = &input[i..i + 4];
    literal_bin.push_str(group_bin);
    i += 4;
  }

  let literal = u64::from_str_radix(&literal_bin, 2).unwrap();
  return (literal, i);
}

fn hex2bin(hex: &str) -> String {
  let mut bin = String::new();
  for i in (0..hex.len()).step_by(2) {
    // println!("{}", i);
    let x = u8::from_str_radix(&hex[i..i + 2], 16).unwrap();
    bin.push_str(&format!("{:08b}", x));
  }
  return bin;
}
