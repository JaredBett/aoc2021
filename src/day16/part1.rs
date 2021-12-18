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
  // length: OperatorPacketLength,
  sub_packets: Vec<Packet>,
}

#[derive(Debug)]
enum OperatorPacketLength {
  Bits(u16),
  Packets(u16),
}

pub fn main() {
  // let input = include_str!("sample1_literal.txt");
  // let input = include_str!("sample2_op1.txt");
  // let input = include_str!("sample3_op2.txt");
  // let input = include_str!("sample4.txt");
  // let input = include_str!("sample5.txt");
  // let input = include_str!("sample6.txt");
  // let input = include_str!("sample7.txt");
  let input = include_str!("input.txt");

  let bin = hex2bin(input);
  // println!("{}", bin);

  let (packet, num_bits) = parse_packet(&bin);
  // dbg!(&packet);
  println!("{}", sum_versions(&packet));
}

fn sum_versions(packet: &Packet) -> usize {
  let mut sum: usize = 0;

  match packet {
    Packet::Literal(literal) => sum += literal.header.version as usize,
    Packet::Operator(operator) => {
      sum += operator.header.version as usize;
      for sub_packet in &operator.sub_packets {
        sum += sum_versions(&sub_packet);
      }
    }
  }

  return sum;
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

    let op_packet = OperatorPacket {
      header,
      sub_packets,
    };
    return (Packet::Operator(op_packet), i);
  }

  // break;
  // }

  // return (packet, i);
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
