use std::io;
fn main() {
    let mut tape = make_tape(200);
    let instr_count: i32 = count_of_instructions();
    let instr: Vec<String> = get_instructions(instr_count);
    println!("Instructions: {:?}", instr);
}

fn input_from_user(input_text: String) -> String{
    let mut input = String::new();
    println!("{}", input_text);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input!");
    input.trim().to_string()
}
fn get_instructions(ici: i32) -> Vec<String> {
    let mut instructions: Vec<String> = Vec::new();
    for _i in 0..ici{
        let input = String::from("Give me the instruction for the turing machine: ");
        let instruction = input_from_user(input);
        instructions.push(instruction);
    }
    instructions
}
fn count_of_instructions() -> i32 {
    let input = String::from("How many instruction has the turing machine: ");
    let instruction_count = input_from_user(input);
    println!("Instruction count: {}", instruction_count);
    let instruction_count_int = instruction_count.parse::<i32>().unwrap();
    instruction_count_int
}

fn make_tape(length: i32) -> Vec<i32> {
    let mut tape = Vec::new();
    for _i in 1..length{
        tape.push(0);
    }
    tape
}
