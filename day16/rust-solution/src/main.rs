use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::{Instant};
use std::collections::HashMap;
use std::collections::HashSet;

fn bool_to_u(b: bool) -> usize {
  if b {
    1
  } else {
    0
  }
}

#[derive(Debug)]
pub struct VM {
  registers: Device,
  op_codes: HashMap<usize, OpCode>,
  instructions: Vec<Instruction>
}

impl VM {
  pub fn load(regs: Device, ops: HashMap<usize, OpCode>, inst: Vec<Instruction>) -> VM {
    VM {
      registers: regs,
      op_codes: ops,
      instructions: inst
    }
  }

  fn execute(&mut self) {
    for inst in self.instructions.iter() {
      let op = self.op_codes.get(&inst.op).unwrap();
      op.apply_op(&mut self.registers, &inst.ins, inst.out).unwrap();
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Device([usize; 4]);

impl Device {
  pub fn new(s: &str) -> Option<Device> {
    let mut regs = s 
      .trim_matches(|c| c == '[' || c == ']')
      .split(", ")
      .flat_map(|reg| str::parse(reg).ok());

    Some(Device([regs.next()?, regs.next()?, regs.next()?, regs.next()?])) 
  }

  pub fn get_reg(&mut self, reg: usize) -> Result<&mut usize, String> {
    match self.0.get_mut(reg) {
      Some(reg) => Ok(reg),
      _ => Err("register not found".to_string())
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OpCode 
{ Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori,
  Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
}

impl OpCode {
  fn variants() -> impl Iterator<Item = OpCode> {
    use self::OpCode::*;

    vec! [Addr, Addi, Mulr, Muli ,Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr].into_iter()
  }

  fn apply_op(&self, d: &mut Device, ins: &[usize; 2], out: usize) -> Result<(), String> {
    use self::OpCode::*;

    let [a, b] = *ins;
    let reg_a = *d.get_reg(a)?;
    let reg_b = *d.get_reg(b)?;

    *d.get_reg(out)? = match *self {
      Addr => reg_a + reg_b,
      Addi => reg_a + b,
      Mulr => reg_a * reg_b,
      Muli => reg_a * b,
      Banr => reg_a & reg_b,
      Bani => reg_a & b,
      Borr => reg_a | reg_b,
      Bori => reg_a | b,
      Gtir => bool_to_u(a > reg_b),
      Gtri => bool_to_u(reg_a > b),
      Gtrr => bool_to_u(reg_a > reg_b),
      Eqir => bool_to_u(a == reg_b),
      Eqri => bool_to_u(reg_a == b),
      Eqrr => bool_to_u(reg_a == reg_b),
      Setr => reg_a,
      Seti => a
    };
    Ok(())
  }
}

#[derive(Debug)]
pub struct Instruction {
  op: usize,
  ins: [usize; 2],
  out: usize
}

impl Instruction {
  pub fn new(s: &str) -> Option<Instruction> {
    let mut input = s.split(" ").flat_map(|d| str::parse(d).ok());

    Some(Instruction {
      op: input.next()?,
      ins: [input.next()?, input.next()?],
      out: input.next()?
    })
  }
}

#[derive(Debug)]
pub struct Data {
  before: Device,
  inst: Instruction,
  after: Device
}

fn main() {
  let now = Instant::now();
  let mut ops: HashMap<usize, HashSet<OpCode>> = HashMap::new();
  let mut result = 0;

  if let Some(tests) = load_tests() {
    for test in tests.iter() {
      let mut found = HashSet::new();

      for op_code in OpCode::variants() {
        let mut test_device = test.before.clone();
        op_code.apply_op(&mut test_device, &test.inst.ins, test.inst.out).expect("Error executing op");

        if test_device == test.after {
          found.insert(op_code);
        }
      }

      if found.len() >= 3 {
        result += 1;
      }

      let matches = ops.entry(test.inst.op).or_insert(found.iter().cloned().collect());
      *matches = matches.intersection(&found).cloned().collect();  
    }

    resolve_ops(&mut ops);
  }

  let op_codes = ops.iter() 
    .map(|(op_code, op)| (*op_code, *op.iter().next().unwrap()))
    .collect::<HashMap<usize, OpCode>>();

  if let Some(instructions) = load_instructions() {
    let mut registers = Device([0, 0, 0, 0]);
    let mut vm = VM::load(registers, op_codes, instructions);
    vm.execute();
    println!("Part2: {}", vm.registers.get_reg(0).unwrap());
  }

  println!("Part1: {}", result);
  println!("{:?}", Instant::now().duration_since(now));
}

fn resolve_ops(ops: &mut HashMap<usize, HashSet<OpCode>>) {
  loop {
    let mut known_ops = HashMap::new();

    for (op, matches) in ops.iter() {
      if matches.len() == 1 {
        known_ops.insert(*op, matches.iter().cloned().collect());
      }
    }
    
    for (op1, matches) in known_ops.iter() {
      for (op2, unknown) in ops.iter_mut() {
        if op1 != op2 {
          *unknown = unknown.difference(&matches).cloned().collect();
        }
      } 
    }
    if known_ops.len() == ops.len() {
      break;
    }
  }
}

fn load_instructions() -> Option<Vec<Instruction>> {
  let f = File::open("part2.txt").expect("Error opening file");
  let lines = BufReader::new(f).lines();
  let mut instructions = Vec::new();

  for line in lines {
    instructions.push(Instruction::new(&line.unwrap())?);
  }

  Some(instructions)
}

fn load_tests() -> Option<Vec<Data>> {
  let f = File::open("part1.txt").expect("Error opening file");
  let mut lines = BufReader::new(f).lines();
  let mut tests: Vec<Data> = Vec::new();

  loop {
    let before = lines.next()?.unwrap();

    if before == "" {
      break;
    }

    let inst = lines.next()?.unwrap();
    let after = lines.next()?.unwrap();
    let b = lines.next()?.unwrap();

    if b != "" {
      break;
    }

    let before = Device::new(before.split(": ").nth(1)?.trim())?;
    let inst = Instruction::new(&inst)?;
    let after = Device::new(after.split(": ").nth(1)?.trim())?;

    tests.push(
      Data {
        before,
        inst,
        after
      }
    )
  }
  Some(tests)
}
