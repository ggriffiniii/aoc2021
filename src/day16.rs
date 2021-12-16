use aoc_runner_derive::aoc;

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let mut msg = Message::new(input);
    sum_version_numbers(&parse_packet(&mut msg))
}

fn sum_version_numbers(packet: &Packet) -> usize {
    packet.version as usize
        + match &packet.lit_or_op {
            LitOrOp::Literal(_) => 0,
            LitOrOp::Op { packets, .. } => packets.iter().map(sum_version_numbers).sum(),
        }
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let mut msg = Message::new(input);
    eval_packet(&parse_packet(&mut msg))
}

fn eval_packet(packet: &Packet) -> usize {
    match &packet.lit_or_op {
        LitOrOp::Literal(lit) => *lit as usize,
        LitOrOp::Op {
            op: Op::Sum,
            packets,
        } => packets.iter().map(eval_packet).sum(),
        LitOrOp::Op {
            op: Op::Product,
            packets,
        } => packets.iter().map(eval_packet).product(),
        LitOrOp::Op {
            op: Op::Min,
            packets,
        } => packets.iter().map(eval_packet).min().unwrap(),
        LitOrOp::Op {
            op: Op::Max,
            packets,
        } => packets.iter().map(eval_packet).max().unwrap(),
        LitOrOp::Op {
            op: Op::Gt,
            packets,
        } => (eval_packet(&packets[0]) > eval_packet(&packets[1])) as usize,
        LitOrOp::Op {
            op: Op::Lt,
            packets,
        } => (eval_packet(&packets[0]) < eval_packet(&packets[1])) as usize,
        LitOrOp::Op {
            op: Op::Eq,
            packets,
        } => (eval_packet(&packets[0]) == eval_packet(&packets[1])) as usize,
    }
}

/// A Message takes a hex string and provides methods for consuming bits from
/// left to right.
struct Message<'a> {
    hex: &'a str,
    bit_cursor: usize,
}

impl<'a> Message<'a> {
    fn new(hex: &'a str) -> Self {
        Message { hex, bit_cursor: 0 }
    }

    /// Consume the next `nbits` bits from the message. `nbits` must be <= 61.
    fn consume(&mut self, nbits: usize) -> u64 {
        assert!(nbits <= 61);
        let first_bit = self.bit_cursor;
        self.bit_cursor += nbits;
        let hex_char_start = first_bit / 4;
        let hex_char_end = (self.bit_cursor + 3) / 4;
        let value = u64::from_str_radix(&self.hex[hex_char_start..hex_char_end], 16).unwrap();
        let right_shift = hex_char_end * 4 - self.bit_cursor;
        let mask = (1 << nbits) - 1;
        (value >> right_shift) & mask
    }

    /// Return the current cursor position. The difference between two cursor
    /// positions will indicate how many bits have been consumed during that
    /// duration.
    fn cursor(&self) -> usize {
        self.bit_cursor
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    lit_or_op: LitOrOp,
}

fn parse_packet(msg: &mut Message) -> Packet {
    let version = msg.consume(3) as u8;
    let typ = msg.consume(3) as u8;
    let lit_or_op = match typ {
        0 => LitOrOp::Op {
            op: Op::Sum,
            packets: parse_op_packets(msg),
        },
        1 => LitOrOp::Op {
            op: Op::Product,
            packets: parse_op_packets(msg),
        },
        2 => LitOrOp::Op {
            op: Op::Min,
            packets: parse_op_packets(msg),
        },
        3 => LitOrOp::Op {
            op: Op::Max,
            packets: parse_op_packets(msg),
        },
        4 => LitOrOp::Literal(parse_literal(msg)),
        5 => LitOrOp::Op {
            op: Op::Gt,
            packets: parse_op_packets(msg),
        },
        6 => LitOrOp::Op {
            op: Op::Lt,
            packets: parse_op_packets(msg),
        },
        7 => LitOrOp::Op {
            op: Op::Eq,
            packets: parse_op_packets(msg),
        },
        _ => panic!("unknown packet type"),
    };
    Packet { version, lit_or_op }
}

fn parse_literal(msg: &mut Message) -> u64 {
    let mut lit = 0;
    for _ in 0..16 {
        let last = msg.consume(1) == 0;
        lit = (lit << 4) | msg.consume(4);
        if last {
            return lit;
        }
    }
    panic!("literal greater than 64 bits")
}

fn parse_op_packets(msg: &mut Message) -> Vec<Packet> {
    let length_type_id = msg.consume(1);
    if length_type_id == 0 {
        let total_bit_length = msg.consume(15) as usize;
        let start_cursor = msg.cursor();
        let mut packets = Vec::new();
        while msg.cursor() - start_cursor < total_bit_length {
            packets.push(parse_packet(msg));
        }
        packets
    } else {
        let num_subpackets = msg.consume(11);
        (0..num_subpackets).map(|_| parse_packet(msg)).collect()
    }
}

#[derive(Debug)]
enum LitOrOp {
    Literal(u64),
    Op { op: Op, packets: Vec<Packet> },
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}
