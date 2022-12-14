#[derive(Debug, Clone)]
enum Packet {
    PacketValue(u32),
    PacketList(Vec<Packet>)
}

impl Packet {
    fn is_divider(&self) -> bool {
        match self {
            Packet::PacketList(l) => {
                match l.as_slice() {
                    [f] => {
                        match f {
                            Packet::PacketList(l) => {
                                match l.as_slice() {
                                    [f] => {
                                        match f {
                                            Packet::PacketValue(2 | 6) => true,
                                            _ => false
                                        }
                                    },
                                    _ => false
                                }
                            },
                            _ => false
                        }
                    },
                    _ => false
                }
            },
            _ => false
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::PacketValue(v) => {write!(f, "{}", v)},
            Packet::PacketList(l) => {
                let a: Vec<_> = l.iter().map(|p| p.to_string()).collect();
                write!(f, "[{}]", a.join(", "))
            }
        }
    }
}

fn compile_packet(packet_data: &str) -> Packet {
    let mut sub_packets = vec![];
    sub_packets.push(Packet::PacketList(vec![]));
    
    let mut val: Option<Packet> = None;
    for c in packet_data.chars() {
        match c {
            '[' => {
                sub_packets.push(Packet::PacketList(vec![]))
            },
            ']' => {
                let mut p = sub_packets.pop().unwrap();
                if let Packet::PacketList(l) = &mut p {
                    if let Some(vp) = val {
                        l.push(vp);
                    }
                    val = Some(p);
                }
            },
            ',' => {
                let p = sub_packets.last_mut().unwrap();
                if let Packet::PacketList(l) = p {
                    if let Some(p) = val {
                        l.push(p);
                        val = None;
                    }
                }
            },
            d => {
                let nd = d.to_string().parse::<u32>().unwrap();
                val = match val {
                    None => Some(Packet::PacketValue(nd)),
                    Some(Packet::PacketValue(v)) => Some(Packet::PacketValue(v * 10 + nd)),
                    _ => panic!("Malformed packet '{}'", packet_data)
                }
            }
        }
    }
    
    val.unwrap()
}

#[allow(dead_code)]
fn flatten_packet(packet: Packet) -> Vec<u32> {
    match packet {
        Packet::PacketList(l) => l.into_iter().flat_map(|p| flatten_packet(p)).collect(),
        Packet::PacketValue(v) => vec![v]
    }
}

fn build_packets(packet_content: &str) -> Vec<(Packet, Packet)> {
    packet_content.lines().collect::<Vec<&str>>().chunks(3)
        .map(
            |chunk| {
                match chunk {
                    [p1, p2, ""] => {
                        (compile_packet(p1), compile_packet(p2))
                    }, 
                    [p1, p2] => {
                        (compile_packet(p1), compile_packet(p2))
                    },
                    _ => panic!("Broken packet pair '{:?}'", chunk)
                }
            }
        ).collect()
}

fn verify_pair_order(pair: (Packet, Packet)) -> Option<bool> {
    // println!("Compare {} vs {}", pair.0, pair.1);
    match pair {
        (Packet::PacketList(l), Packet::PacketList(r)) => {
            let left_shorter = match l.len().cmp(&r.len()) {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(false)
            };
            let mut inner = None;
            for ip in l.into_iter().zip(r.into_iter()) {
                match verify_pair_order(ip) {
                    Some(b) => {
                        inner = Some(b);
                        break; 
                    },
                    None => ()
                } 
            }
            match inner {
                Some(b) => Some(b),
                None => left_shorter
            }
        },
        (Packet::PacketValue(v), Packet::PacketList(l)) => {
            verify_pair_order((Packet::PacketList(vec![Packet::PacketValue(v)]), Packet::PacketList(l)))
        },
        (Packet::PacketList(l), Packet::PacketValue(v)) => {
            verify_pair_order((Packet::PacketList(l), Packet::PacketList(vec![Packet::PacketValue(v)])))
        },
        (Packet::PacketValue(l), Packet::PacketValue(r)) => {
            match l.cmp(&r) {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Equal => None,
                std::cmp::Ordering::Greater => Some(false)
            }
        }
    }
}

fn verify_packets(packet_content: &str) -> u32 {
    let packets = build_packets(packet_content);
    packets.into_iter()
        .enumerate()
        .filter_map(
            |(i, pair) | {
                // println!("Pair {}", i + 1);
                let a = if let Some(true) = verify_pair_order(pair) {
                    Some(i as u32 + 1)
                } else {
                    None
                };
                // println!("Order '{:?}'", a);
                // println!();
                a
            }
        )
        .sum()
        
}

pub fn verify_packets_order(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => verify_packets(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn order_and_select(packet_content: &str) -> u32 {
    let mut packets: Vec<_> = build_packets(packet_content).into_iter()
        .flat_map(|(p1, p2)| vec![p1, p2])
        .collect();
    packets.push(Packet::PacketList(vec![Packet::PacketList(vec![Packet::PacketValue(2)])]));
    packets.push(Packet::PacketList(vec![Packet::PacketList(vec![Packet::PacketValue(6)])]));

    packets.sort_by(
        |a, b| {
            match verify_pair_order((a.clone(), b.clone())) {
                Some(true) => std::cmp::Ordering::Less,
                None => std::cmp::Ordering::Equal,
                Some(false) => std::cmp::Ordering::Greater
            }
        }
    );
    packets.iter()
        .enumerate()
        .filter_map(
            |(i, p)| {
                if p.is_divider() {
                    Some((i + 1) as u32)
                } else {
                    None
                }
            }
        )
        .product()
}

pub fn order_packets_and_select_distress(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => order_and_select(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP1: &str = 
r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn test_input1() {
        assert_eq!(verify_packets(TEST_INP1), 13)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(order_and_select(TEST_INP1), 140)
    }
}