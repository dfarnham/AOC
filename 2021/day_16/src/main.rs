use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use std::error::Error;
use std::io::{self, Write};

const PACKET_HEADER: usize = 6;
const MIN_PACKET_BITS: usize = 11; // PACKET_HEADER + 5-bit NUM

// Literal value packets encode a single binary number.
// To do this, the binary number is padded with leading zeroes until its length is a multiple of four bits,
// and then it is broken into groups of four bits. Each group is prefixed by a 1 bit except the last group,
// which is prefixed by a 0 bit. These groups of five bits immediately follow the packet header.
// For example, the hexadecimal string D2FE28 becomes:
//
// 110100 10111 11110 00101000
// VVVTTT AAAAA BBBBB CCCCC
//
// 110100101111111000101000
// VVVTTTAAAAABBBBBCCCCC
//
// Below each bit is a label indicating its purpose:
//
//  The three bits labeled V (110) are the packet version, 6.
//  The three bits labeled T (100) are the packet type ID, 4, which means the packet is a literal value.
//  The five bits labeled A (10111) start with a 1 (not the last group, keep reading) and contain the first four bits of the number, 0111.
//  The five bits labeled B (11110) start with a 1 (not the last group, keep reading) and contain four more bits of the number, 1110.
//  The five bits labeled C (00101) start with a 0 (last group, end of packet) and contain the last four bits of the number, 0101.
//  The three unlabeled 0 bits at the end are extra due to the hexadecimal representation and should be ignored.
//
// So, this packet represents a literal value with binary representation 011111100101, which is 2021 in decimal.
//
// Every other type of packet (any packet with a type ID other than 4) represent an operator that performs some calculation on one
// or more sub-packets contained within. Right now, the specific operations aren't important; focus on parsing the hierarchy of sub-packets.
//
// An operator packet contains one or more packets. To indicate which subsequent binary data represents its sub-packets, an operator
// packet can use one of two modes indicated by the bit immediately after the packet header; this is called the length type ID:
//
//  If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
//  If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
//
// Finally, after the length type ID bit and the 15-bit or 11-bit field, the sub-packets appear.
//
// For example, here is an operator packet (hexadecimal string 38006F45291200) with length type ID 0 that contains two sub-packets:
//
// 00111000000000000110111101000101001010010001001000000000
// VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
//
//     The three bits labeled V (001) are the packet version, 1.
//     The three bits labeled T (110) are the packet type ID, 6, which means the packet is an operator.
//     The bit labeled I (0) is the length type ID, which indicates that the length is a 15-bit number representing the number of bits in the sub-packets.
//     The 15 bits labeled L (000000000011011) contain the length of the sub-packets in bits, 27.
//     The 11 bits labeled A contain the first sub-packet, a literal value representing the number 10.
//     The 16 bits labeled B contain the second sub-packet, a literal value representing the number 20.
//
// After reading 11 and 16 bits of sub-packet data, the total length indicated in L (27) is reached, and so parsing of this packet stops.
//
// Literal values (type ID 4) represent a single number as described above. The remaining type IDs are more interesting:
//
//   Packets with type ID 0 are sum packets - their value is the sum of the values of their sub-packets.
//   If they only have a single sub-packet, their value is the value of the sub-packet.
//
//   Packets with type ID 1 are product packets - their value is the result of multiplying together the values of their sub-packets.
//   If they only have a single sub-packet, their value is the value of the sub-packet.
//
//   Packets with type ID 2 are minimum packets - their value is the minimum of the values of their sub-packets.
//
//   Packets with type ID 3 are maximum packets - their value is the maximum of the values of their sub-packets.
//
//   Packets with type ID 5 are greater than packets - their value is 1 if the value of the first sub-packet is greater than
//   the value of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
//
//   Packets with type ID 6 are less than packets - their value is 1 if the value of the first sub-packet is less than the value
//   of the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.
//
//   Packets with type ID 7 are equal to packets - their value is 1 if the value of the first sub-packet is equal to the value of
//   the second sub-packet; otherwise, their value is 0. These packets always have exactly two sub-packets.

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    id: TypeId,
    op: Op,
    sub_packets: Option<Vec<Packet>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeId {
    Literal(usize), // number of bits to encode NUM
    Operator(Payload),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    NUM(u64),
    SUM,
    PROD,
    MIN,
    MAX,
    GT,
    LT,
    EQ,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Payload {
    BitLen(usize),       // number of bits in embedded packet
    SubPacketLen(usize), // number of following packets
}

fn bits2num(bits: &[u8]) -> u64 {
    bits.iter().fold(0, |acc, b| acc << 1 | *b as u64)
}

fn decode_packet(bits: &[u8]) -> Packet {
    let version = bits2num(&bits[0..3]) as u8;
    let type_id = bits2num(&bits[3..6]) as u8;

    let opcode = match type_id {
        0 => Op::SUM,
        1 => Op::PROD,
        2 => Op::MIN,
        3 => Op::MAX,
        4 => Op::NUM(0), // set below
        5 => Op::GT,
        6 => Op::LT,
        7 => Op::EQ,
        _ => panic!("invalid type_id = {type_id}"),
    };

    let (op, id) = match opcode {
        Op::NUM(_) => {
            let mut nibbles = vec![];
            for (i, bit) in bits.iter().skip(PACKET_HEADER).enumerate() {
                if i % 5 == 0 {
                    if *bit == 0 {
                        // last 4 bits
                        nibbles.extend(&bits[(i + PACKET_HEADER + 1)..=(i + PACKET_HEADER + 4)]);
                        break;
                    }
                    continue; // skip every 5th bit
                }
                nibbles.push(*bit);
            }
            (
                Op::NUM(bits2num(&nibbles)),
                TypeId::Literal(nibbles.len() + nibbles.len() / 4),
            )
        }
        _ => match bits[PACKET_HEADER] == 0 {
            true => (
                opcode,
                TypeId::Operator(Payload::BitLen(
                    bits2num(&bits[(PACKET_HEADER + 1)..=(PACKET_HEADER + 15)]) as usize,
                )),
            ),
            false => (
                opcode,
                TypeId::Operator(Payload::SubPacketLen(
                    bits2num(&bits[(PACKET_HEADER + 1)..=(PACKET_HEADER + 11)]) as usize,
                )),
            ),
        },
    };

    let sub_packets: Option<Vec<Packet>> = match id {
        TypeId::Literal(_) => None,
        TypeId::Operator(Payload::SubPacketLen(_)) => None,
        TypeId::Operator(Payload::BitLen(n)) => {
            Some(get_packets(&bits[(PACKET_HEADER + 16)..(PACKET_HEADER + 16 + n)]))
        }
    };

    Packet {
        version,
        id,
        op,
        sub_packets,
    }
}

fn get_packets(bits: &[u8]) -> Vec<Packet> {
    let mut offset = 0;
    let mut packets = vec![];
    while offset + MIN_PACKET_BITS <= bits.len() {
        let packet = decode_packet(&bits[offset..]);
        offset += match packet.id {
            TypeId::Literal(n) => PACKET_HEADER + n,
            TypeId::Operator(Payload::BitLen(n)) => PACKET_HEADER + n + 16,
            TypeId::Operator(Payload::SubPacketLen(_)) => PACKET_HEADER + 12,
        };
        packets.push(packet);
    }
    packets
}

fn get_bits(msg: &str) -> Vec<u8> {
    let mut bits = vec![];
    for c in msg.chars() {
        let nibble = u8::from_str_radix(&c.to_string(), 16).expect("hex conversion failed");
        bits.push(nibble >> 3 & 1);
        bits.push(nibble >> 2 & 1);
        bits.push(nibble >> 1 & 1);
        bits.push(nibble & 1);
    }
    bits
}

fn apply_operator(op: &Op, values: &[u64]) -> u64 {
    let mut stack = values.to_vec();

    match op {
        Op::SUM => stack.iter().sum::<u64>(),
        Op::PROD => stack.iter().product::<u64>(),
        Op::MIN => *stack.iter().min().expect("min() failed"),
        Op::MAX => *stack.iter().max().expect("max() failed"),
        Op::GT => match stack.pop() < stack.pop() {
            true => 1,
            false => 0,
        },
        Op::LT => match stack.pop() > stack.pop() {
            true => 1,
            false => 0,
        },
        Op::EQ => match stack.pop() == stack.pop() {
            true => 1,
            false => 0,
        },
        _ => panic!("invalid opcode = {op:?}"),
    }
}

// counts the actual number of packets needed for an input packet range
// taking into account SubPacketLen packets extend the count
fn packets_needed(n: usize, packets: &[Packet]) -> usize {
    let mut target = n;
    let mut index = 0;
    while index < target {
        if let TypeId::Operator(Payload::SubPacketLen(c)) = packets[index].id {
            target += c;
        }
        index += 1;
    }
    index
}

fn eval(packets: &[Packet]) -> Vec<u64> {
    let mut stack = vec![];
    let mut index = 0;
    while index < packets.len() {
        let opcode = &packets[index].op;
        match packets[index].id {
            TypeId::Literal(_) => {
                stack.push(match opcode {
                    Op::NUM(n) => *n,
                    _ => panic!("Literal Op must be type Op::NUM, op = {opcode:?}"),
                });
            }
            TypeId::Operator(Payload::BitLen(_)) => {
                stack.push(apply_operator(
                    opcode,
                    &eval(packets[index].sub_packets.as_ref().expect("corrupt Payload::BitLen")),
                ));
            }
            TypeId::Operator(Payload::SubPacketLen(n)) => {
                let count = packets_needed(n, &packets[(index + 1)..]);
                stack.push(apply_operator(
                    opcode,
                    &eval(&packets[(index + 1)..(index + 1 + count)]),
                ));
                index += count;
            }
        };
        index += 1;
    }
    stack
}

fn solution1(packets: &[Packet]) -> u64 {
    let mut total = 0;
    for p in packets {
        total += p.version as u64;
        if let Some(sp) = &p.sub_packets {
            total += solution1(sp);
        }
    }
    total
}

fn solution2(packets: &[Packet]) -> u64 {
    eval(packets)[0]
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_trimmed_data_lines::<String>(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    let bits = get_bits(&puzzle_lines[0]);
    let packets = get_packets(&bits);
    //println!("packets = {:#?}", packets);
    writeln!(stdout, "Answer Part 1 = {:?}", solution1(&packets))?;
    writeln!(stdout, "Answer Part 2 = {:?}", solution2(&packets))?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<String> {
        let file = std::path::PathBuf::from(filename);
        read_trimmed_data_lines::<String>(Some(&file)).unwrap()
    }
    #[test]
    fn test1() {
        let data = "C200B40A82";
        let bits = get_bits(data);
        let packets = get_packets(&bits);

        assert_eq!(packets.len(), 3);
        assert_eq!(
            packets[0],
            Packet {
                version: 6,
                id: TypeId::Operator(Payload::SubPacketLen(2)),
                op: Op::SUM,
                sub_packets: None
            }
        );
        assert_eq!(
            packets[1],
            Packet {
                version: 6,
                id: TypeId::Literal(5),
                op: Op::NUM(1),
                sub_packets: None
            }
        );
        assert_eq!(
            packets[2],
            Packet {
                version: 2,
                id: TypeId::Literal(5),
                op: Op::NUM(2),
                sub_packets: None
            }
        );

        assert_eq!(solution1(&packets), 6 + 6 + 2);
        assert_eq!(solution2(&packets), 1 + 2);
    }

    #[test]
    fn test2() {
        let data = "04005AC33890";
        let bits = get_bits(data);
        let packets = get_packets(&bits);

        assert_eq!(packets.len(), 1);
        assert_eq!(
            packets[0],
            Packet {
                version: 0,
                id: TypeId::Operator(Payload::BitLen(22)),
                op: Op::PROD,
                sub_packets: Some(vec![
                    Packet {
                        version: 5,
                        id: TypeId::Literal(5),
                        op: Op::NUM(6),
                        sub_packets: None
                    },
                    Packet {
                        version: 3,
                        id: TypeId::Literal(5),
                        op: Op::NUM(9),
                        sub_packets: None
                    },
                ])
            }
        );

        assert_eq!(solution1(&packets), 5 + 3);
        assert_eq!(solution2(&packets), 6 * 9);
    }

    #[test]
    fn test3() {
        let data = "9C0141080250320F1802104A08";
        let bits = get_bits(data);
        let packets = get_packets(&bits);

        assert_eq!(packets.len(), 1);
        assert_eq!(
            packets[0],
            Packet {
                version: 4,
                id: TypeId::Operator(Payload::BitLen(80)),
                op: Op::EQ,
                sub_packets: Some(vec![
                    Packet {
                        version: 2,
                        id: TypeId::Operator(Payload::SubPacketLen(2)),
                        op: Op::SUM,
                        sub_packets: None,
                    },
                    Packet {
                        version: 2,
                        id: TypeId::Literal(5),
                        op: Op::NUM(1),
                        sub_packets: None
                    },
                    Packet {
                        version: 4,
                        id: TypeId::Literal(5),
                        op: Op::NUM(3),
                        sub_packets: None
                    },
                    Packet {
                        version: 6,
                        id: TypeId::Operator(Payload::SubPacketLen(2)),
                        op: Op::PROD,
                        sub_packets: None,
                    },
                    Packet {
                        version: 0,
                        id: TypeId::Literal(5),
                        op: Op::NUM(2),
                        sub_packets: None
                    },
                    Packet {
                        version: 2,
                        id: TypeId::Literal(5),
                        op: Op::NUM(2),
                        sub_packets: None
                    },
                ])
            }
        );

        assert_eq!(solution1(&packets), 4 + 2 + 2 + 4 + 6 + 0 + 2);
        assert_eq!(solution2(&packets), 1);
    }

    #[test]
    fn test4() {
        let data = "880086C3E88112";
        let bits = get_bits(data);
        let packets = get_packets(&bits);

        assert_eq!(packets.len(), 1);
        assert_eq!(
            packets[0],
            Packet {
                version: 4,
                id: TypeId::Operator(Payload::BitLen(33)),
                op: Op::MIN,
                sub_packets: Some(vec![
                    Packet {
                        version: 5,
                        id: TypeId::Literal(5),
                        op: Op::NUM(7),
                        sub_packets: None
                    },
                    Packet {
                        version: 6,
                        id: TypeId::Literal(5),
                        op: Op::NUM(8),
                        sub_packets: None
                    },
                    Packet {
                        version: 0,
                        id: TypeId::Literal(5),
                        op: Op::NUM(9),
                        sub_packets: None
                    },
                ])
            }
        );

        assert_eq!(solution1(&packets), 4 + 5 + 6 + 0);
        assert_eq!(solution2(&packets), *[7, 8, 9].iter().min().unwrap());
    }

    #[test]
    fn test5() {
        let data = "38006F45291200";
        let bits = get_bits(data);
        let packets = get_packets(&bits);

        assert_eq!(packets.len(), 1);
        assert_eq!(
            packets[0],
            Packet {
                version: 1,
                id: TypeId::Operator(Payload::BitLen(27)),
                op: Op::LT,
                sub_packets: Some(vec![
                    Packet {
                        version: 6,
                        id: TypeId::Literal(5),
                        op: Op::NUM(10),
                        sub_packets: None
                    },
                    Packet {
                        version: 2,
                        id: TypeId::Literal(10),
                        op: Op::NUM(20),
                        sub_packets: None
                    },
                ])
            }
        );

        assert_eq!(solution1(&packets), 1 + 6 + 2);
        assert_eq!(solution2(&packets), 1);
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        let bits = get_bits(&data[0]);
        let packets = get_packets(&bits);
        assert_eq!(solution1(&packets), 6);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let bits = get_bits(&data[0]);
        let packets = get_packets(&bits);
        assert_eq!(solution1(&packets), 866);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let bits = get_bits(&data[0]);
        let packets = get_packets(&bits);
        assert_eq!(solution2(&packets), 2021);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let bits = get_bits(&data[0]);
        let packets = get_packets(&bits);
        assert_eq!(solution2(&packets), 1392637195518);
    }
}
