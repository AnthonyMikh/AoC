use std::collections::HashMap;

type Signal = u16;
type Shift = u8;

#[derive(Clone, Copy)]
enum Input<Idx> {
    Wire(Idx),
    Signal(Signal),
}

impl<'a> From<&'a str> for Input<&'a str> {
    fn from(s: &'a str) -> Self {
        s.parse().map(Self::Signal).unwrap_or(Self::Wire(s))
    }
}

#[derive(Clone, Copy)]
enum Gate<Idx> {
    Wire(Idx),
    Constant(Signal),
    And(Input<Idx>, Idx),
    Or(Idx, Idx),
    LeftShift(Idx, Shift),
    RightShift(Idx, Shift),
    Not(Idx),
}

impl<'a> Gate<&'a str> {
    fn parse(s: &'a str) -> (Self, &'a str) {
        let (gate, output) = s.split_once(" -> ").unwrap();
        (Self::parse_inner(gate), output)
    }

    fn parse_inner(gate: &'a str) -> Self {
        if let Ok(value) = gate.parse() {
            return Self::Constant(value);
        }
        if let Some(input) = gate.strip_prefix("NOT ") {
            return Self::Not(input);
        }
        if let Some((a, b)) = gate.split_once(" AND ") {
            return Self::And(a.into(), b);
        }
        if let Some((a, b)) = gate.split_once(" OR ") {
            return Self::Or(a, b);
        }
        if let Some((a, b)) = gate.split_once(" LSHIFT ") {
            return Self::LeftShift(a, b.parse().unwrap());
        }
        if let Some((a, b)) = gate.split_once(" RSHIFT ") {
            return Self::RightShift(a, b.parse().unwrap());
        }
        Self::Wire(gate)
    }
}

fn eval<'a>(
    wire: &'a str,
    gates: &HashMap<&'a str, Gate<&'a str>>,
    values: &mut HashMap<&'a str, Signal>,
) -> Signal {
    if let Some(&answer) = values.get(&wire) {
        return answer;
    }

    let answer = match gates[&wire] {
        Gate::Wire(w) => eval(w, gates, values),
        Gate::Constant(v) => v,
        Gate::And(a, b) => {
            let left = match a {
                Input::Wire(w) => eval(w, gates, values),
                Input::Signal(s) => s,
            };
            left & eval(b, gates, values)
        }
        Gate::Or(a, b) => eval(a, gates, values) | eval(b, gates, values),
        Gate::LeftShift(wire, shift) => {
            eval(wire, gates, values).checked_shl(shift.into()).unwrap()
        }
        Gate::RightShift(wire, shift) => {
            eval(wire, gates, values).checked_shr(shift.into()).unwrap()
        }
        Gate::Not(wire) => !eval(wire, gates, values),
    };

    values.insert(wire, answer);
    answer
}

const ANSWER_WIRE: &str = "a";
const OVERRIDE_WIRE: &str = "b";

fn parse_gates(s: &str) -> HashMap<&str, Gate<&str>> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(Gate::parse)
        .map(|(gate, wire)| (wire, gate))
        .collect()
}

fn solve_both(input: &str) -> Signal {
    let gates = parse_gates(input);
    let mut values = HashMap::with_capacity(gates.len());
    let override_ = eval(ANSWER_WIRE, &gates, &mut values);
    values.clear();
    values.insert(OVERRIDE_WIRE, override_);
    eval(ANSWER_WIRE, &gates, &mut values)
}
