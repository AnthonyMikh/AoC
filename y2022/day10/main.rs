type Num = i32;

enum Instruction {
    Noop,
    Addx(Num),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        if s == "noop" {
            return Self::Noop;
        }
        let val = s.strip_prefix("addx ").unwrap();
        Self::Addx(val.parse().unwrap())
    }
}

#[derive(Debug)]
enum State {
    Decode,
    Addx(Num),
}

struct Cpu<'a> {
    x: Num,
    memory: &'a [Instruction],
    state: State,
    pc: usize,
}

impl<'a> Cpu<'a> {
    fn new(memory: &'a [Instruction]) -> Self {
        Self {
            x: 1,
            memory,
            state: State::Decode,
            pc: 0,
        }
    }

    fn tick(&mut self) -> bool {
        match self.state {
            State::Addx(n) => {
                self.x += n;
                self.pc += 1;
                self.state = State::Decode;
            }
            State::Decode => {
                let Some(ins) = self.memory.get(self.pc) else { return false };
                match ins {
                    Instruction::Noop => self.pc += 1,
                    &Instruction::Addx(n) => self.state = State::Addx(n),
                }
            }
        }
        true
    }
}

struct Screen {
    position: Num,
    pixels: String,
}

impl Screen {
    const HEIGHT: usize = 6;
    const WIDTH: usize = 40;

    fn new() -> Self {
        Self {
            position: 0,
            pixels: String::with_capacity(Self::WIDTH * Self::HEIGHT),
        }
    }

    fn tick(&mut self, x: Num) {
        if let -1..=1 = self.position - x {
            self.pixels.push('#');
        } else {
            self.pixels.push('.');
        }
        self.position += 1;
        if self.position == Self::WIDTH as Num {
            self.position = 0;
        }
    }

    fn render(&self) {
        for row in 0..Self::HEIGHT {
            let offset = row * Self::WIDTH;
            println!("{}", &self.pixels[offset..offset + Self::WIDTH]);
        }
    }
}

fn solve(input: &str) -> (Num, Screen) {
    let instructions = input.lines().map(Instruction::parse).collect::<Vec<_>>();
    let mut ret = 0;
    let mut cpu = Cpu::new(&instructions);
    let mut screen = Screen::new();
    for cycle in 1.. {
        if cycle % 40 == 20 {
            ret += cycle * cpu.x;
        }
        screen.tick(cpu.x);
        if !cpu.tick() {
            break;
        }
    }
    (ret, screen)
}

fn main() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    let (answer, screen) = solve(input);
    println!("{answer}");
    screen.render();
}
