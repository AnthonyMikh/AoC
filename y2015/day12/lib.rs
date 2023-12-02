use serde_json::Value as Json;

#[derive(Default)]
struct Adder {
    total: i64,
}

impl Adder {
    fn visit(&mut self, value: &Json, reject: &dyn Fn(&Json) -> bool) {
        use Json::*;

        if reject(value) {
            return;
        }

        match value {
            Null | Bool(..) | String(..) => {}
            Number(n) => self.total += n.as_i64().unwrap(),
            Array(arr) => arr.iter().for_each(|v| self.visit(v, reject)),
            Object(o) => o.values().for_each(|v| self.visit(v, reject)),
        }
    }
}

fn solve_both(input: &str) -> (i64, i64) {
    let document = serde_json::from_str::<Json>(input).unwrap();

    let mut adder = Adder::default();
    adder.visit(&document, &|_| false);
    let first = adder.total;

    let mut adder = Adder::default();
    let reject_with_red_prop = |v: &Json| {
        v.as_object()
            .map_or(false, |o| o.values().any(|prop| prop == "red"))
    };
    adder.visit(&document, &reject_with_red_prop);
    let second = adder.total;

    (first, second)
}
