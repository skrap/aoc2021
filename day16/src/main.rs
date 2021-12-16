fn main() {
    let input = include_str!("input");
    dbg!(part1(input));
    dbg!(part2(input));
}

fn part1(input: &str) -> usize {
    let packet = parse(input);
    packet.sum_versions()
}

fn part2(input: &str) -> usize {
    let packet = parse(input);
    packet.eval()
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        type_id: usize,
        version: usize,
        value: Vec<Packet>,
    },
}

impl Packet {
    fn sum_versions(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator { version, value, .. } => {
                value.iter().map(|v| v.sum_versions()).sum::<usize>() + *version
            }
        }
    }

    fn eval(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { type_id, value, .. } => {
                let mut evals = value.iter().map(Packet::eval);
                match type_id {
                    0 => evals.sum(),
                    1 => evals.product(),
                    2 => evals.min().unwrap(),
                    3 => evals.max().unwrap(),
                    5 => (evals.next().unwrap() > evals.next().unwrap()).into(),  
                    6 => (evals.next().unwrap() < evals.next().unwrap()).into(),
                    7 => (evals.next().unwrap() == evals.next().unwrap()).into(),
                    _ => unimplemented!(),
                }
            }
        }
    }
}

struct BitGetter {
    bytes: Vec<u8>,
    bit_pos: usize,
}

impl BitGetter {
    fn from_str(input: &str) -> Self {
        let mut bytes = Vec::new();
        let mut nibbles = input.trim().chars().map(|c| c.to_digit(16).unwrap() as u8);
        while let Some(nibble) = nibbles.next() {
            let byte = nibble << 4 | nibbles.next().unwrap();
            bytes.push(byte);
        }
        Self { bytes, bit_pos: 0 }
    }

    fn get(&mut self, bit_len: usize) -> usize {
        let mut result = 0usize;
        for i in 0..bit_len {
            let byte = self.bytes[(self.bit_pos + i) / 8];
            let bitnum = u8::try_from((self.bit_pos + i) % 8).unwrap();
            let bit = (byte >> (7 - bitnum)) & 0b1;
            result = (result << 1) | usize::from(bit);
        }
        self.bit_pos += bit_len;
        result
    }
}

#[test]
fn test_bits() {
    let input = "D2FE28";
    let mut bits = BitGetter::from_str(input);
    assert_eq!(bits.get(3), 6);
    assert_eq!(bits.get(3), 4);
    assert_eq!(bits.get(1), 1);
    assert_eq!(bits.get(4), 7);
    assert_eq!(bits.get(1), 1);
    assert_eq!(bits.get(4), 14);
    assert_eq!(bits.get(1), 0);
    assert_eq!(bits.get(4), 5);
    assert_eq!(bits.get(3), 0);

    bits = BitGetter::from_str(input);
    let packet = parse_1(&mut bits);
    assert_eq!(
        packet,
        Packet::Literal {
            version: 6,
            value: 2021
        }
    );
}

fn parse(input: &str) -> Packet {
    let mut bits = BitGetter::from_str(input);
    parse_1(&mut bits)
}

fn parse_1(bits: &mut BitGetter) -> Packet {
    let version = bits.get(3);
    let type_id = bits.get(3);

    match type_id {
        4 => {
            let mut value = 0;
            loop {
                let more = bits.get(1);
                value = (value << 4) | bits.get(4);
                if more == 0 {
                    break;
                }
            }
            Packet::Literal { version, value }
        }
        _ => {
            let len_type = bits.get(1);
            if len_type == 0 {
                let payload_bitlen = bits.get(15);
                let mut payload = Vec::new();
                let payload_end = bits.bit_pos + payload_bitlen;
                while bits.bit_pos < payload_end {
                    payload.push(parse_1(bits));
                }
                Packet::Operator {
                    type_id,
                    version,
                    value: payload,
                }
            } else {
                let payload_packets = bits.get(11);
                let mut payload = Vec::new();
                for _i in 0..payload_packets {
                    payload.push(parse_1(bits));
                }
                Packet::Operator {
                    type_id,
                    version,
                    value: payload,
                }
            }
        }
    }
}
