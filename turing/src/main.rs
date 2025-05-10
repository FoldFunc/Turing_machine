use std::io;
fn main() {
    let mut tape = make_tape(200);
    let instr_count: i32 = count_of_instructions();
    let instr: Vec<(String, String)> = get_instructions(instr_count);
    println!("{:?}", instr);
    let instr_good: bool = validate_instructions(&instr);
    println!("Is valid: {:?}", instr_good);
}

fn validate_instructions(instructions: &Vec<(String, String)>) -> bool {
    for (ins_0, ins_1) in instructions {
        //println!("Instructions if 0: {}", ins_0);
        //println!("Instructions if 1: {}", ins_1);
        if ins_0.len() > 3 || ins_1.len() > 3 {
            return false;
        }
        /*
         * The input should look somthing like this:
         * 1R0
         * This means that if the machine sees 1 on the tape it should go Right
         * and then listen to the instruction 0
         */
        for c in ins_0.chars() {
            if let Some(first_letter) = ins_0.chars().nth(0) {
                if c == first_letter {
                    println!("{} is the first letter", c);
                    if !c.is_digit(10) {
                        return false;
                    }
                    if c != '1' || c != '0' {
                        return false;
                    }
                }
            }
            if let Some(second_letter) = ins_0.chars().nth(1) {
                if c ==  second_letter{
                    println!("{} is the second letter", c);
                    if c != 'R' || c != 'L' {
                       return false; 
                    }
                }
            }
            if let Some(third_letter) = ins_0.chars().nth(2) {
                if c == third_letter{
                    println!("{} is the third letter", c);
                    if !c.is_digit(10) {
                        return false;
                    }
                    if c != '1' || c != '0' {
                        return false;
                    }
                }
            }
            
        }
    }
    return true;
}


// Takes a String and asks user to enter something in command line
fn input_from_user(input_text: String) -> String{
    let mut input = String::new();
    println!("{}", input_text);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input!");
    input.trim().to_string()
}
// Gets a specific instruction from the user
fn get_instructions(ici: i32) -> Vec<(String, String)> {
    let mut instructions = Vec::new();
    for _i in 0..ici{
        let input = String::from("Give me the instruction for the turing machine if machine sees 0: ");
        let instruction_0 = input_from_user(input);
        let input = String::from("Give me the instruction for the turing machine if machine sees 1: ");
        let instruction_1 = input_from_user(input);
        instructions.push((instruction_0, instruction_1))
    }
    instructions
}
// Gets amount of instructions that the turing machine will use
fn count_of_instructions() -> i32 {
    let input = String::from("How many instruction has the turing machine: ");
    let instruction_count = input_from_user(input);
    println!("Instruction count: {}", instruction_count);
    let instruction_count_int = instruction_count.parse::<i32>().unwrap();
    instruction_count_int
}
// Makes a tape of zeros to turing machine to edit
fn make_tape(length: i32) -> Vec<i32> {
    let mut tape = Vec::new();
    for _i in 1..length{
        tape.push(0);
    }
    tape
}
