use core::panic;
use itertools::Itertools;
use std::path::Path;

mod input;

#[derive(Debug)]
enum PacketType {
    Literal(i64),
    Operator(OperatorType, Vec<Packet>),
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Min,
    Max,
    GT,
    LT,
    Eq,
}

#[derive(Debug)]
struct Packet {
    version: i64,
    packet_type: PacketType,
}

impl Packet {
    pub fn new(version: i64, packet_type: PacketType) -> Self {
        Packet {
            version,
            packet_type,
        }
    }
}

fn get_operator_type(type_id: i64) -> OperatorType {
    match type_id {
        0 => OperatorType::Sum,
        1 => OperatorType::Product,
        2 => OperatorType::Min,
        3 => OperatorType::Max,
        5 => OperatorType::GT,
        6 => OperatorType::LT,
        7 => OperatorType::Eq,
        _ => unreachable!("Unexpected Type Id"),
    }
}

pub fn solve(file_path: &Path) {
    let hexs = input::read_hexstring(file_path)
        .unwrap_or_else(|_| panic!("Unable to read file {:?}", file_path));
    let packets = parse_hexs(&hexs);
    println!(
        "Packet Decoder Version Sum: {}",
        compute_version_sums(&packets)
    );
    println!(
        "Packet Decode Packet Value {}",
        compute_packet_value(&packets[0])
    )
}

fn compute_version_sums(packets: &[Packet]) -> i64 {
    packets
        .iter()
        .map(|p| {
            p.version
                + match &p.packet_type {
                    PacketType::Literal(_) => 0,
                    PacketType::Operator(_, subpackets) => compute_version_sums(subpackets),
                }
        })
        .sum()
}

fn compute_packet_value(packet: &Packet) -> i64 {
    match &packet.packet_type {
        PacketType::Literal(v) => *v,
        PacketType::Operator(op, values) => match op {
            OperatorType::Sum => values.iter().map(compute_packet_value).sum(),
            OperatorType::Product => values.iter().map(compute_packet_value).product(),
            OperatorType::Min => values.iter().map(compute_packet_value).min().unwrap(),
            OperatorType::Max => values.iter().map(compute_packet_value).max().unwrap(),
            OperatorType::GT => {
                (compute_packet_value(&values[0]) > compute_packet_value(&values[1])) as i64
            }
            OperatorType::LT => {
                (compute_packet_value(&values[0]) < compute_packet_value(&values[1])) as i64
            }
            OperatorType::Eq => {
                (compute_packet_value(&values[0]) == compute_packet_value(&values[1])) as i64
            }
        },
    }
}

#[allow(unstable_name_collisions)]
fn parse_hexs(hexs: &str) -> Vec<Packet> {
    let sbits: String = hexs
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|d| format!("{:04b}", d))
        .intersperse("".to_owned())
        .collect();

    let bits = sbits.as_bytes();

    parse_packets(bits, 1).0
}

fn parse_packets(bits: &[u8], max_count: i64) -> (Vec<Packet>, usize) {
    let mut packets = vec![];
    let mut start = 0;
    for _ in 0..max_count {
        if start + 6 > bits.len() {
            break;
        }

        let version = parse_u8(&bits[start..start + 3]);
        let type_id = parse_u8(&bits[start + 3..start + 6]);

        if type_id == 4 {
            let (value, end) = parse_literal(&bits[start + 6..]);
            let packet = Packet::new(version, PacketType::Literal(value));
            start = start + 6 + end;
            packets.push(packet);
        } else {
            let (subpackets, end) = parse_operator(&bits[start + 6..]);
            let packet = Packet::new(
                version,
                PacketType::Operator(get_operator_type(type_id), subpackets),
            );
            start = start + 6 + end;
            packets.push(packet);
        }
    }

    (packets, start)
}

fn parse_u8(bits: &[u8]) -> i64 {
    let chars: Vec<char> = bits.iter().map(|u| *u as char).collect();
    let s: String = chars.iter().collect();
    u64::from_str_radix(&s, 2).unwrap() as i64
}

fn parse_literal(bits: &[u8]) -> (i64, usize) {
    let mut start = 0;
    let mut buffers: Vec<u8> = vec![];

    while start + 5 <= bits.len() {
        let buffer = &bits[start..start + 5];

        buffer.iter().skip(1).for_each(|bit| buffers.push(*bit));

        if buffer[0] == b'0' {
            break;
        }
        start += 5;
    }

    (parse_u8(&buffers), start + 5)
}

fn parse_operator(bits: &[u8]) -> (Vec<Packet>, usize) {
    let len_type_id = bits[0];
    if len_type_id == b'0' {
        let n = parse_u8(&bits[1..16]);
        let (packets, end) = parse_packets(&bits[16..16 + n as usize], i64::MAX);
        (packets, end + 16)
    } else if len_type_id == b'1' {
        let n = parse_u8(&bits[1..12]);
        let (packets, end) = parse_packets(&bits[12..], n);
        (packets, end + 12)
    } else {
        panic!("Unknown len_type_id")
    }
}

#[test]
fn test_parse_literal() {
    let s = "101111111000101000";
    let k = parse_literal(s.as_bytes());
    assert!(k.0 == 2021);
    assert!(k.1 == 15)
}

#[test]
fn test_parse_literal_2() {
    let s = "1000100100";
    let k = parse_literal(s.as_bytes());
    assert!(k.0 == 20);
    assert!(k.1 == s.len())
}

#[test]
fn test_parse_u8() {
    assert!(parse_u8("110".as_bytes()) == 6);
    assert!(parse_u8("111".as_bytes()) == 7)
}

#[test]
fn test_literal_packet_parse() {
    let literal_packet = "110100101111111000101000";
    let (packets, end) = parse_packets(literal_packet.as_bytes(), 1);
    assert!(packets.len() == 1);
    assert!(end == literal_packet.len() - 3);
}

#[test]
fn test_operator_type_1() {
    let s = "11101110000000001101010000001100100000100011000001100000";
    let (packets, end) = parse_packets(s.as_bytes(), 1);
    assert!(packets.len() == 1);
    assert!(end == s.len() - 5);
    assert!(packets[0].version == 7);
}

#[test]
fn test_operator_type_0() {
    let s = "00111000000000000110111101000101001010010001001000000000";
    let (packets, end) = parse_packets(s.as_bytes(), 1);
    assert!(packets.len() == 1);
    assert!(end == s.len() - 7);
    assert!(packets[0].version == 1);
}

#[test]
fn test_parsing_2() {
    let s = "110100010100101001000100100";
    let (packets, end) = parse_packets(s.as_bytes(), 2);
    assert!(packets.len() == 2);
    assert!(end == s.len());
}

#[test]
fn test_parse_hex() {
    let hex = "8A004A801A8002F478";
    let packets = parse_hexs(hex);
    assert!(compute_version_sums(&packets) == 16)
}

#[test]
fn test_parse_hex_2() {
    let hex = "620080001611562C8802118E34";
    let packets = parse_hexs(hex);
    assert!(compute_version_sums(&packets) == 12)
}

#[test]
fn test_parse_hex_3() {
    let hex = "C0015000016115A2E0802F182340";
    let packets = parse_hexs(hex);
    assert!(compute_version_sums(&packets) == 23)
}

#[test]
fn test_parse_hex_4() {
    let hex = "A0016C880162017C3686B18A3D4780";
    let packets = parse_hexs(hex);
    assert!(compute_version_sums(&packets) == 31)
}
