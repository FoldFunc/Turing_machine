use std::io;

fn main() {
    let mut tape = vec![0; 200];
    let instr_count = get_instruction_count();
    let instructions = get_instructions(instr_count);

    if !validate_instructions(&instructions) {
        println!("The machine input is not valid");
        return;
    }

    execute_instructions(&mut tape, &instructions);
}

fn get_instruction_count() -> usize {
    let input = input_from_user("How many instructions does the Turing machine have: ");
    input.parse::<usize>().expect("Please enter a valid number")
}

fn input_from_user(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_instructions(count: usize) -> Vec<(String, String)> {
    let mut instructions = Vec::new();
    for i in 0..count {
        let ins0 = input_from_user(&format!("Instruction {} if tape has 0: ", i));
        let ins1 = input_from_user(&format!("Instruction {} if tape has 1: ", i));
        instructions.push((ins0, ins1));
    }
    instructions
}

fn is_valid_instruction(instr: &str) -> bool {
    if instr.len() != 3 {
        return false;
    }
    let chars: Vec<char> = instr.chars().collect();
    (chars[0] == '0' || chars[0] == '1') &&
    (chars[1] == 'R' || chars[1] == 'L') &&
    (chars[2].is_ascii_digit() && chars[2].to_digit(10).unwrap() < 10)
}

fn validate_instructions(instructions: &Vec<(String, String)>) -> bool {
    instructions.iter().all(|(a, b)| is_valid_instruction(a) && is_valid_instruction(b))
}

fn execute_instructions(tape: &mut Vec<i32>, instructions: &Vec<(String, String)>) {
    let mut curr_pos: usize = 100;
    let mut instr_idx: usize = 0;

    while instr_idx < instructions.len() {
        let cell = tape[curr_pos];
        let instr_str = if cell == 0 {
            &instructions[instr_idx].0
        } else {
            &instructions[instr_idx].1
        };

        println!("Executing instruction at {}: {}", instr_idx, instr_str);

        let chars: Vec<char> = instr_str.chars().collect();
        let write_val = chars[0].to_digit(10).unwrap() as i32;
        let direction = chars[1];
        let next_instr = chars[2].to_digit(10).unwrap() as usize;

        tape[curr_pos] = write_val;

        match direction {
            'L' => {
                if curr_pos == 0 {
                    println!("Reached beginning of tape. Halting.");
                    break;
                }
                curr_pos -= 1;
            },
            'R' => {
                curr_pos += 1;
                if curr_pos >= tape.len() {
                    println!("Reached end of tape. Halting.");
                    break;
                }
            },
            _ => {
                println!("Invalid direction. Halting.");
                break;
            }
        }

        instr_idx = next_instr;
        println!("Tape: {:?}", &tape[95..105]); // Show small window for clarity
    }

    println!("Final tape (excerpt): {:?}", &tape[95..105]);
}

