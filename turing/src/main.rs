use std::io;
fn main() {
    let mut tape = Vec::new();
    for _i in 1..100 {
        tape.push(0);
    }
    let mut input = String::from("How many instruction has the turing machine: ");
    let instruction_count = input_from_user(input);
    println!("Instruction count: {}", instruction_count);
    let instruction_count_int = instruction_count.parse::<i32>().unwrap();
    let mut instructions: Vec<String> = Vec::new();
    for _i in 0..instruction_count_int {
        input = String::from("Give me the instruction for the turing machine: ");
        let instruction = input_from_user(input);
        instructions.push(instruction);
    }
    println!("Instructions for a turing machine: {:?}", instructions);
}

fn input_from_user(input_text: String) -> String{
    let mut input = String::new();
    println!("{}", input_text);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input!");
    input.trim().to_string()
}
