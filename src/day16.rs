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
            LitOrOp::Sum(packets)
            | LitOrOp::Product(packets)
            | LitOrOp::Min(packets)
            | LitOrOp::Max(packets)
            | LitOrOp::Gt(packets)
            | LitOrOp::Lt(packets)
            | LitOrOp::Eq(packets) => packets.iter().map(sum_version_numbers).sum(),
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
        LitOrOp::Sum(packets) => packets.iter().map(eval_packet).sum(),
        LitOrOp::Product(packets) => packets.iter().map(eval_packet).product(),
        LitOrOp::Min(packets) => packets.iter().map(eval_packet).min().unwrap(),
        LitOrOp::Max(packets) => packets.iter().map(eval_packet).max().unwrap(),
        LitOrOp::Gt(packets) => (eval_packet(&packets[0]) > eval_packet(&packets[1])) as usize,
        LitOrOp::Lt(packets) => (eval_packet(&packets[0]) < eval_packet(&packets[1])) as usize,
        LitOrOp::Eq(packets) => (eval_packet(&packets[0]) == eval_packet(&packets[1])) as usize,
    }
}

struct Message<'a> {
    hex: &'a str,
    bit_cursor: usize,
}

impl<'a> Message<'a> {
    fn new(hex: &'a str) -> Self {
        Message { hex, bit_cursor: 0 }
    }

    fn consume(&mut self, nbits: usize) -> u64 {
        assert!(nbits <= 64);
        let first_bit = self.bit_cursor;
        self.bit_cursor += nbits;
        let hex_char_start = first_bit / 4;
        let hex_char_end = self.bit_cursor / 4 + 1;
        let value = u64::from_str_radix(&self.hex[hex_char_start..hex_char_end], 16).unwrap();
        let right_shift = hex_char_end * 4 - self.bit_cursor;
        let mask = (1 << nbits) - 1;
        (value >> right_shift) & mask
    }

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
        0 => LitOrOp::Sum(parse_op_packets(msg)),
        1 => LitOrOp::Product(parse_op_packets(msg)),
        2 => LitOrOp::Min(parse_op_packets(msg)),
        3 => LitOrOp::Max(parse_op_packets(msg)),
        4 => LitOrOp::Literal(parse_literal(msg)),
        5 => LitOrOp::Gt(parse_op_packets(msg)),
        6 => LitOrOp::Lt(parse_op_packets(msg)),
        7 => LitOrOp::Eq(parse_op_packets(msg)),
        _ => panic!("unknown packet type"),
    };
    Packet { version, lit_or_op }
}

fn parse_literal(msg: &mut Message) -> u64 {
    let mut lit = 0;
    loop {
        let last = msg.consume(1) == 0;
        lit <<= 4;
        lit |= msg.consume(4);
        if last {
            return lit;
        }
    }
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
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Gt(Vec<Packet>),
    Lt(Vec<Packet>),
    Eq(Vec<Packet>),
}
