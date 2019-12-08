#[derive(Debug)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfNz,
    JumpIfZ,
    JumpLt,
    JumpEq,
    Exit,
}

impl OpCode {
    fn from(chars: &mut Vec<char>) -> Self {
        let opcode = chars.pop().unwrap().to_digit(10).unwrap()
            + (chars.pop().unwrap_or('0').to_digit(10).unwrap()) * 10;

        match opcode {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfNz,
            6 => OpCode::JumpIfZ,
            7 => OpCode::JumpLt,
            8 => OpCode::JumpEq,
            99 => OpCode::Exit,
            x => panic!("Unrecognized opcode {}", x),
        }
    }
}

#[derive(Debug)]
enum ReferenceType {
    Direct,
    Indirect,
}

impl ReferenceType {
    fn from(c: char) -> Self {
        match c {
            '1' => ReferenceType::Direct,
            '0' => ReferenceType::Indirect,
            x => panic!("Unrecognized reference type {}", x),
        }
    }
}

#[derive(Default, Clone)]
struct Computer {
    memory: Vec<i64>,
    index: usize,
    instruction_chars: Vec<char>,
}

impl Computer {
    fn new_from_file(filename: &str) -> Self {
        Computer {
            memory: std::fs::read_to_string(filename)
                .unwrap()
                .split(',')
                .map(|input| input.parse::<i64>().unwrap())
                .collect(),
            ..Default::default()
        }
    }

    fn get_next_value(&mut self) -> i64 {
        let val = match ReferenceType::from(self.instruction_chars.pop().unwrap_or('0')) {
            ReferenceType::Indirect => self.memory[self.memory[self.index] as usize],
            ReferenceType::Direct => self.memory[self.index],
        };
        self.index += 1;
        val
    }

    fn write(&mut self, val: i64) {
        let dest = self.memory[self.index] as usize;
        self.memory[dest] = val;
        self.index += 1;
    }

    fn get_instruction(&mut self) -> Vec<char> {
        let instruction = self.memory[self.index].to_string().chars().collect();
        self.index += 1;
        instruction
    }

    fn run(mut self, mut inputs: Vec<i64>) -> Result<i64, i64> {

        println!("Inputs {:?}", inputs);
        self.index = 0;

        loop {
            self.instruction_chars = self.get_instruction();
            let opcode = OpCode::from(&mut self.instruction_chars);
            println!("{:?}", opcode);

            match opcode {
                OpCode::Add => {
                    let val = self.get_next_value() + self.get_next_value();
                    self.write(val);
                }
                OpCode::Multiply => {
                    let val = self.get_next_value() * self.get_next_value();
                    self.write(val);
                }
                OpCode::Input => {
                    self.write(inputs.remove(0));
                }
                OpCode::Output => {
                    let v1 = self.get_next_value();
                    return Err(v1); //inputs.push(v1);
                }
                OpCode::JumpIfZ => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    if v1 == 0 {
                        println!("Jump!");
                        self.index = v2 as usize;
                    }
                }
                OpCode::JumpIfNz => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    if v1 != 0 {
                        println!("Jump!");
                        self.index = v2 as usize;
                    }
                }
                OpCode::JumpLt => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    self.write(if v1 < v2 { 1 } else { 0 });
                }
                OpCode::JumpEq => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    self.write(if v1 == v2 { 1 } else { 0 });
                }
                OpCode::Exit => {
                    panic!("Exiting!");
                    return Ok(inputs.pop().unwrap());
                }
            };
        }
    }
}

fn main() {
    let computer = Computer::new_from_file("inputs.txt");
    let mut orderings = Vec::<Vec<i64>>::new();

    for ii in 5..10 {
        for jj in 5..10 {
            for kk in 5..10 {
                for ll in 5..10 {
                    for mm in 5..10 {
                        let phase_settings: Vec<i64> = vec![ii, jj, kk, ll, mm];
                        let set: std::collections::HashSet<_> = phase_settings.iter().collect();
                        if set.len() != 5 {
                            continue;
                        }

                        orderings.push(phase_settings);
                    }
                }
            }
        }
    }

    let amps = vec![computer.clone(), computer.clone(), computer.clone(), computer.clone(), computer.clone()];

    let max = orderings
        .into_iter()
        .map(|mut ordering| {
            println!("Ordering {:?}", ordering);

            let mut index = 0;
            let mut next_input = 0;
            
            loop {
                let mut full_inputs = if index < 5 {
                    vec![ordering[index % 5], next_input]
                } else {
                    vec![ordering[index % 5], next_input]
                };
                //full_inputs.append(vec![next_input]);
                match amps[index % 5]
                    .clone()
                    .run(full_inputs) {
                        Err(x) => {
                            next_input = x;
                        },
                        Ok(x) => {
                            println!("-> {:?}", x);
                            return x;
                        }
                    }
                index = (index + 1);
            }
            println!("-> {:?}", next_input);
            next_input
        })
        .max();

    println!("Max output {}", max.unwrap());
}
