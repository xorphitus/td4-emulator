use num_derive::{FromPrimitive, ToPrimitive};

struct Register {
    pc: u8,
    a: u8,
    b: u8,
    c: u8,
}

struct Port {
    i: u8,
    o: u8,
}

struct Rom {
    vec: Vec<u8>,
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive)]
pub enum Opcode {
    AddA = 0b0000,  // ADD A, Im
    AddB = 0b0101,  // ADD B, Im
    MovA = 0b0011,  // MOV A, Im
    MovB = 0b0111,  // MOV B, Im
    MovAB = 0b0001, // MOV A, B
    MovBA = 0b0100, // MOV B, A
    Jmp = 0b1111,   // JMP Im
    Jnc = 0b1110,   // JNC Im
    InA = 0b0010,   // IN A
    InB = 0b0110,   // IN B
    Out = 0b1011,   // OUT Im
    OutB = 0b1001,  // OUT B
}

fn fetch(register: &mut Register, rom: Rom) -> u8 {
    let pc = register.pc as usize;
    if rom.vec.len() < pc {
        return 0;
    }
    let code = rom.vec[pc];
    register.pc += 1;
    code
}

fn decode(code: u8) -> (Opcode, u8) {
    let op = code >> 4;
    let im = code & 0x0f;

    match num_traits::FromPrimitive::from_u8(op) {
        Some(op) => (op, im),
        _ => panic!("Error: Not implemented opcode: {:04b}", op),
    }
}

fn execute(register: &mut Register, port: Port, rom: Rom) {
    let code = fetch(register, rom);
    let (op, im) = decode(code);

    match op {
        Opcode::AddA => add_a(register, im),
        Opcode::AddB => add_b(register, im),
        Opcode::MovA => mov_a(register, im),
        Opcode::MovB => mov_b(register, im),
        Opcode::MovAB => mov_ab(register),
        Opcode::MovBA => mov_ba(register),
        Opcode::Jmp => jmp(register, im),
        Opcode::Jnc => jmc(register, im),
        Opcode::InA => in_a(register, port),
        Opcode::InB => in_b(register, port),
        Opcode::Out => out(register, port, im),
        Opcode::OutB => out_b(register, port),
    };
}

fn add_a(register: &mut Register, im: u8) {}
fn add_b(register: &mut Register, im: u8) {}
fn mov_a(register: &mut Register, im: u8) {}
fn mov_b(register: &mut Register, im: u8) {}
fn mov_ab(register: &mut Register) {}
fn mov_ba(register: &mut Register) {}
fn jmp(register: &mut Register, im: u8) {}
fn jmc(register: &mut Register, im: u8) {}
fn in_a(register: &mut Register, port: Port) {}
fn in_b(register: &mut Register, port: Port) {}
fn out(register: &mut Register, port: Port, im: u8) {}
fn out_b(register: &mut Register, port: Port) {}

fn main() {
    println!("Hello, world!");
}
