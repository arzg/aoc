use crate::parsing::{extract_digits, extract_lowercase, tag};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Circuit {
    pub instructions: Vec<Instruction>,
}

impl Circuit {
    pub fn emulate(&self) -> HashMap<&str, u16> {
        let instructions = self
            .instructions
            .iter()
            .map(|Instruction { wire, signal }| (wire.as_str(), signal.clone()))
            .collect();

        let mut cache = HashMap::with_capacity(self.instructions.len());

        self.instructions
            .iter()
            .map(|Instruction { wire, signal }| {
                (wire.as_str(), signal.emulate(&instructions, &mut cache))
            })
            .collect()
    }

    pub fn override_and_clear(
        &self,
        wire_signals: HashMap<&str, u16>,
        from: &str,
        to: &str,
    ) -> u16 {
        let instructions = self
            .instructions
            .iter()
            .map(|Instruction { wire, signal }| (wire.as_str(), signal.clone()))
            .collect();

        let mut cache = {
            let from_value = *wire_signals.get(from).unwrap();
            let mut cache = HashMap::with_capacity(wire_signals.len());
            cache.insert(to, from_value);
            cache
        };

        self.instructions
            .iter()
            .find_map(|Instruction { wire, signal }| {
                if wire == from {
                    Some(signal.emulate(&instructions, &mut cache))
                } else {
                    None
                }
            })
            .unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    signal: Signal,
    wire: String,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, signal) = Signal::new(s)?;
        let s = tag(" -> ", s)?;
        let (s, wire) = extract_lowercase(s)?;

        if !s.is_empty() {
            return Err("parser did not consume entire input".to_string());
        }

        Ok(Self {
            signal,
            wire: wire.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Signal {
    WithGate {
        lhs: SingleSignal,
        rhs: SingleSignal,
        gate: Gate,
    },
    Not {
        signal: SingleSignal,
    },
    Single(SingleSignal),
}

impl Signal {
    fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_bin_op(s)
            .or_else(|_| Self::new_not(s))
            .or_else(|_| Self::new_single(s))
    }

    fn new_bin_op(s: &str) -> Result<(&str, Self), String> {
        let (s, lhs) = SingleSignal::new(s)?;
        let s = tag(" ", s)?;

        let (s, op) = Gate::new(s)?;
        let s = tag(" ", s)?;

        let (s, rhs) = SingleSignal::new(s)?;

        Ok((s, Self::WithGate { lhs, rhs, gate: op }))
    }

    fn new_not(s: &str) -> Result<(&str, Self), String> {
        let s = tag("NOT ", s)?;
        let (s, signal) = SingleSignal::new(s)?;

        Ok((s, Self::Not { signal }))
    }

    fn new_single(s: &str) -> Result<(&str, Self), String> {
        let (s, single_signal) = SingleSignal::new(s)?;
        Ok((s, Self::Single(single_signal)))
    }

    fn emulate<'a>(
        &'a self,
        wire_signals: &'a HashMap<&'a str, Signal>,
        cache: &mut HashMap<&'a str, u16>,
    ) -> u16 {
        match self {
            Signal::WithGate { lhs, rhs, gate } => gate.apply_to(
                lhs.emulate(wire_signals, cache),
                rhs.emulate(wire_signals, cache),
            ),
            Signal::Not { signal } => !signal.emulate(wire_signals, cache),
            Signal::Single(single_signal) => single_signal.emulate(wire_signals, cache),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SingleSignal {
    Literal(u16),
    Wire(String),
}

impl SingleSignal {
    fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_literal(s).or_else(|_| Self::new_wire(s))
    }

    fn new_literal(s: &str) -> Result<(&str, Self), String> {
        let (s, literal) = extract_digits(s)?;
        Ok((s, Self::Literal(literal.parse().unwrap())))
    }

    fn new_wire(s: &str) -> Result<(&str, Self), String> {
        let (s, wire) = extract_lowercase(s)?;
        Ok((s, Self::Wire(wire.to_string())))
    }

    fn emulate<'a>(
        &'a self,
        wire_signals: &'a HashMap<&'a str, Signal>,
        cache: &mut HashMap<&'a str, u16>,
    ) -> u16 {
        match self {
            SingleSignal::Literal(n) => *n,
            SingleSignal::Wire(wire) => cache.get(wire.as_str()).copied().unwrap_or_else(|| {
                let emulated_signal = wire_signals
                    .get(wire.as_str())
                    .unwrap()
                    .emulate(wire_signals, cache);
                cache.insert(wire.as_str(), emulated_signal);

                emulated_signal
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Gate {
    And,
    Or,
    LShift,
    RShift,
}

impl Gate {
    fn new(s: &str) -> Result<(&str, Self), String> {
        tag("AND", s)
            .map(|s| (s, Self::And))
            .or_else(|_| tag("OR", s).map(|s| (s, Self::Or)))
            .or_else(|_| tag("LSHIFT", s).map(|s| (s, Self::LShift)))
            .or_else(|_| tag("RSHIFT", s).map(|s| (s, Self::RShift)))
    }

    fn apply_to(&self, x: u16, y: u16) -> u16 {
        match self {
            Self::And => x & y,
            Self::Or => x | y,
            Self::LShift => x << y,
            Self::RShift => x >> y,
        }
    }
}

#[cfg(test)]
mod emulation_tests {
    use super::*;

    #[test]
    fn emulate_circuit() {
        let circuit = Circuit {
            instructions: vec![
                "123 -> x".parse().unwrap(),
                "456 -> y".parse().unwrap(),
                "x AND y -> d".parse().unwrap(),
                "x OR y -> e".parse().unwrap(),
                "x LSHIFT 2 -> f".parse().unwrap(),
                "y RSHIFT 2 -> g".parse().unwrap(),
                "NOT x -> h".parse().unwrap(),
                "NOT y -> i".parse().unwrap(),
            ],
        };

        let wire_signals = circuit.emulate();

        assert_eq!(
            {
                let mut signals = HashMap::new();

                signals.insert("d", 72);
                signals.insert("e", 507);
                signals.insert("f", 492);
                signals.insert("g", 114);
                signals.insert("h", 65412);
                signals.insert("i", 65079);
                signals.insert("x", 123);
                signals.insert("y", 456);

                signals
            },
            wire_signals,
        );
    }

    #[test]
    fn emulate_circuit_with_wire_usage_before_definition() {
        let circuit = Circuit {
            instructions: vec!["a -> b".parse().unwrap(), "10 -> a".parse().unwrap()],
        };

        let wire_signals = circuit.emulate();

        assert_eq!(
            {
                let mut signals = HashMap::new();
                signals.insert("a", 10);
                signals.insert("b", 10);

                signals
            },
            wire_signals,
        );
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::*;

    #[test]
    fn parse_instruction_with_literal() {
        assert_eq!(
            "123 -> x".parse(),
            Ok(Instruction {
                signal: Signal::Single(SingleSignal::Literal(123)),
                wire: "x".to_string(),
            }),
        );
    }

    #[test]
    fn parse_literal_signal() {
        assert_eq!(
            Signal::new("1234"),
            Ok(("", Signal::Single(SingleSignal::Literal(1234)))),
        );
    }

    #[test]
    fn parse_signal_with_gate() {
        assert_eq!(
            Signal::new("x AND y"),
            Ok((
                "",
                Signal::WithGate {
                    lhs: SingleSignal::Wire("x".to_string()),
                    rhs: SingleSignal::Wire("y".to_string()),
                    gate: Gate::And,
                },
            )),
        );
    }

    #[test]
    fn parse_not_signal() {
        assert_eq!(
            Signal::new("NOT e"),
            Ok((
                "",
                Signal::Not {
                    signal: SingleSignal::Wire("e".to_string()),
                },
            )),
        );
    }

    #[test]
    fn parse_or_gate() {
        assert_eq!(Gate::new("OR"), Ok(("", Gate::Or)));
    }
}
