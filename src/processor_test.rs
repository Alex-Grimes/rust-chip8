use super::*;
const START_PC: usize = 0xF00;
const NEXT_PC: usize = START_PC + OPCODE_SIZE;
const SKIPPED_PC: usize = START_PC + (OPCODE_SIZE * 2);
fn build_processor() -> Processor {
    let mut processor = Processor::new();
    processor.pc = START_PC;
    processor.v = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7];
    processor
}
#[test]
fn test_initial_state() {
    let processor = Processor::new();
    assert_eq!(processor.pc, 0x200);
    assert_eq!(processor.sp, 0);
    assert_eq!(processor.stack, [0; 16]);

    assert_eq!(processor.ram[0..5], [0xF0, 0x90, 0x90, 0x90, 0xF0]);

    assert_eq!(
        processor.ram[FONT_SET.len() - 5..FONT_SET.len()],
        [0xF0, 0x80, 0xF0, 0x80, 0x80]
    );
}

#[test]
fn test_load_data() {
    let mut processor = Processor::new();
    processor.load(vec![1, 2, 3]);
    assert_eq!(processor.ram[0x200], 1);
    assert_eq!(processor.ram[0x201], 2);
    assert_eq!(processor.ram[0x202], 3);
}

//CLS - Clear the display
#[test]
fn test_op_00e0() {
    let mut processor = build_processor();
    processor.vram = [[128; CHIP8_WIDTH]; CHIP8_HEIGHT];
    processor.run_opcode(0x00E0);

    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            assert_eq!(processor.vram[y][x], 0);
        }
    }
    assert_eq!(processor.pc, NEXT_PC);
}

//RET - Return from a subroutine
#[test]
fn test_op_00ee() {
    let mut processor = build_processor();
    processor.sp = 5;
    processor.stack[4] = 0x6666;
    processor.run_opcode(0x00EE);
    assert_eq!(processor.pc, 0x6666);
    assert_eq!(processor.sp, 4);
}

//JP addr - Jump to location nnn
#[test]
fn test_op_1nnn() {
    let mut processor = build_processor();
    processor.run_opcode(0x1666);
    assert_eq!(processor.pc, 0x0666);
}

//CALL addr - Call subroutine at nnn
#[test]
fn test_op_2nnn() {
    let mut processor = build_processor();
    processor.run_opcode(0x2666);
    assert_eq!(processor.pc, 0x0666);
    assert_eq!(processor.sp, 1);
    assert_eq!(processor.stack[0], NEXT_PC);
}

//SE Vx, byte - Skip next instruction if Vx = kk
#[test]
fn test_op_3xkk() {
    let mut processor = build_processor();
    processor.run_opcode(0x3201);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.run_opcode(0x3200);
    assert_eq!(processor.pc, NEXT_PC);
}

//SNE Vx, byte - Skip next instruction if Vx != kk
#[test]
fn test_op_4xkk() {
    let mut processor = build_processor();
    processor.run_opcode(0x4200);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.run_opcode(0x4201);
    assert_eq!(processor.pc, NEXT_PC);
}

//SE Vx, Vy - Skip next instruction if Vx = Vy
#[test]
fn test_op_5xy0() {
    let mut processor = build_processor();
    processor.run_opcode(0x5540);
    assert_eq!(processor.pc, SKIPPED_PC);
    let mut processor = build_processor();
    processor.run_opcode(0x5500);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD Vx, byte - Set Vx = kk
#[test]
fn test_op_6xkk() {
    let mut processor = build_processor();
    processor.run_opcode(0x65ff);
    assert_eq!(processor.v[5], 0xff);
    assert_eq!(processor.pc, NEXT_PC);
}

//ADD Vx, byte - Set Vx = Vx + kk
#[test]
fn test_op_7xkk() {
    let mut processor = build_processor();
    processor.run_opcode(0x75f0);
    assert_eq!(processor.v[5], 0xf2);
    assert_eq!(processor.pc, NEXT_PC);
}

// LD Vx, Vy - Set Vx = Vy
#[test]
fn test_op_8xy0() {
    let mut processor = build_processor();
    processor.run_opcode(0x8050);
    assert_eq!(processor.v[0], 0x02);
    assert_eq!(processor.pc, NEXT_PC);
}

fn check_math(v1: u8, v2: u8, op: u16, result: u8, vf: u8) {
    let mut processor = build_processor();
    processor.v[0] = v1;
    processor.v[1] = v2;
    processor.v[0x0F] = 0;
    processor.run_opcode(0x8010 + op);
    assert_eq!(processor.v[0], result);
    assert_eq!(processor.v[0x0F], vf);
    assert_eq!(processor.pc, NEXT_PC);
}

// OR Vx, Vy - Set Vx = Vx OR Vy
#[test]
fn test_op_8xy1() {
    check_math(0x0F, 0xF0, 1, 0);
}

// AND Vx, Vy - Set Vx = Vx AND Vy
#[test]
fn test_op_8xy2() {
    check_math(0x0F, 0xFF, 2, 0x0F, 0);
}

// XOR Vx, Vy - Set Vx = Vx XOR Vy
#[test]
fn test_op_8xy3() {
    check_math(0x0F, 0xF0, 3, 0xFF, 0);
}

// ADD Vx, Vy - Set Vx = Vx + Vy, set VF = carry
#[test]
fn test_op_8xy4() {
    check_math(0x0F, 0xF0, 4, 0xFF, 0);
    check_math(0xFF, 0xFF, 4, 0xFE, 1);
}

//Sub Vx, Vy - Set Vx = Vx - Vy, set VF = NOT borrow
#[test]
fn test_op_8xy5() {
    check_math(0x0F, 0xF0, 5, 0x1F, 1);
    check_math(0x0F, 0x0F, 5, 0x00, 0);
}

// SHR Vx {, Vy} - Set Vx = Vx SHR 1
#[test]
fn test_op_8xy6() {
    check_math(0x0F, 0x01, 4, 0x1E, 0);
    check_math(0xFF, 0xFF, 4, 0xFE, 1);
}
