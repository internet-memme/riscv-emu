
struct core{
    regs: [u32;32],
    pc: u32,
    next_pc: u32,
    memory:Vec<u8>
}

fn main() {
    let programm: Vec<u32> = vec![0b0000000_00010_00001_000_00011_0110011];
    let mut core = core{
        regs: [0;32],
        pc: 0,
        next_pc: 0,
        memory: vec![0;1024]
    };
    //set regs 1 and 2
    load_prog(&mut core, programm);
    core.regs[1] = 1;
    core.regs[2] = 2;
    run(&mut core)
}

fn load_prog(core:&mut core, prog: Vec<u32>){
    let mut mem_addr:usize = 0;
    for instr in prog {
        core.memory[mem_addr + 0] = (instr >>  0) as u8;
        core.memory[mem_addr + 1] = (instr >>  8) as u8;
        core.memory[mem_addr + 2] = (instr >> 16) as u8;
        core.memory[mem_addr + 3] = (instr >> 24) as u8;
        mem_addr += 4;
    }
}

fn run(core: &mut core) {
    loop {
        //fetch instruction
        let curr_instr = fetch_instruction(core);
        core.next_pc = core.pc + 4;
        exec_instr(core, curr_instr);
        core.pc = core.next_pc;
        println!("{:?} pc:{}" , core.regs, core.pc);
    }
}

fn fetch_instruction(core: &mut core) -> u32 {
    let mut instr:u32 = 0;
    for i in 0..4 {
        instr += ((core.memory[(core.pc+i) as usize] as u32)) << i*8;
    }
    instr
}

fn exec_instr(core: &mut core, instr: u32) {
    match get_opcode(instr) {
        0b0110011 => exec_rtype(core, instr), // r-type
        _ => panic!{}
    }
}


fn n_bits(n: u32) -> u32 {
    (1 << n) - 1
}

fn get_opcode(instr: u32) -> u32 {
    instr & n_bits(7)
}

fn get_rd(instr: u32) -> u32 {
    (instr >> 7) & n_bits(5)
}

fn get_funct3(instr: u32) -> u32 {
    (instr >> 12) & n_bits(3)
}

fn get_rs1(instr: u32) -> u32 {
    (instr >> 15) & n_bits(5)
}

fn get_rs2(instr: u32) -> u32 {
    (instr >> 20) & n_bits(5)
}

fn get_funct7(instr: u32) -> u32 {
    (instr >> 25) & n_bits(7)
}

fn exec_rtype(core: &mut core, instr: u32) {
    println!{"parsing rtype"}
    const ADD:u32  = 0b000;
    const SUB:u32  = 0b000;
    const SLL:u32  = 0b001;
    const SLT:u32  = 0b010;
    const SLTU:u32 = 0b011;
    const XOR:u32  = 0b100;
    const SRL:u32  = 0b101;
    const SRA:u32  = 0b101;
    const OR:u32   = 0b110;
    const AND:u32  = 0b111;
    
    let rd = get_rd(instr);
    let funct3 = get_funct3(instr);
    let rs1 = get_rs1(instr);
    let rs2 = get_rs2(instr);
    let funct7 = get_funct7(instr);
    match (funct3, funct7) {
        (ADD,0) => add(core, rs1, rs2, rd),
        // (ADD,0b0100000) => {}, // SUB
        // (SLL,_) => {},
        // (SLT,_) => {},
        // (SLTU,_) => {},
        // (XOR,_) => {},
        // (SRL,0) => {},
        // (SRA,0b0100000) => {},
        // (OR,_) => {},
        // (AND,_) => {},
        (_,_) => panic!{}
    }
}

fn exec_jtype(core: core, instr: u32) {
    let rd = get_rd(instr);
}

fn get_reg(core: &core, reg: u32) -> u32 {
    core.regs[reg as usize % 32] 
}

fn set_reg(core: &mut core, reg:u32,  val:u32) {
    if reg != 0 {core.regs[reg as usize % 32] = val} 
}

fn add (core: &mut core, rs1:u32, rs2:u32, rd:u32){
    println!("add {rs1} {rs2} into {rd}");
    set_reg(core, rd,
        get_reg(core, rs1) + get_reg(core, rs2))
}
