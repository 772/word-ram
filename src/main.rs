use regex::Regex;

fn main() {
    let mut w: usize;
    let limit = 8;
    loop {
        println!("Define the integer w:");
        let raw_input = get_input().parse::<usize>();
        w = match raw_input {
            Ok(number) => number,
            Err(..) => 0,
        };
        if w >= 1 && w <= limit {
            break;
        } else {
            println!("Error: Please choose a number from 1 to {}.", limit);
        }
    }
    let u: usize = 2_usize.pow(w as u32) - 1;
    let mut vec = vec![0; u + 1];
    println!(
        "w = {} bit\nUniverse = {{0, 1, ... {u}}}\nRegisters = {{R0, R1, ... R{u}}}",
        w,
        u = u
    );
    println!("12 commands:\n\tADD Ri\n\tSUB Ri\n\tMUL Ri\n\tDIV Ri\n\tMOD Ri\n\tAND Ri\n\tOR Ri\n\tXOR Ri\n\tHALT\n\tRi = n\n\tLOAD Ri\n\tSTORE Ri");
    loop {
        let command: String = get_input();
        let re = Regex::new(r"\d+").unwrap();
        if re.is_match(&command) {
            let caps = re.captures(&command).unwrap();
            let number = caps.get(0).unwrap().as_str().parse::<usize>().unwrap();
            if number > u {
                println!("Error: {} is higher than {}.", number, u);
                continue;
            }
        }
        if command == "HALT" {
            break;
        } else if command.starts_with('R') {
            let re = Regex::new(r"R(\d+)\s?=\s?(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                let second_number = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                if second_number > u {
                    println!("Error: {} is higher than {}.", second_number, u);
                    continue;
                }
                vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()] = second_number;
            } else {
                println!("Error: Wrong syntax on Ri = n.");
                continue;
            }
        } else if command.starts_with("ADD") {
            let re = Regex::new(r"ADD R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[0] = (vec[0] + vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()])
                    % (u + 1);
            } else {
                println!("Error: Wrong syntax on ADD Ri.");
                continue;
            }
        } else if command.starts_with("SUB") {
            let re = Regex::new(r"SUB R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[0] = (vec[0] - vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()])
                    % (u + 1);
            } else {
                println!("Error: Wrong syntax on SUB Ri.");
                continue;
            }
        } else if command.starts_with("MUL") {
            let re = Regex::new(r"MUL R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[0] = (vec[0] * vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()])
                    % (u + 1);
            } else {
                println!("Error: Wrong syntax on MUL Ri.");
                continue;
            }
        } else if command.starts_with("DIV") {
            let re = Regex::new(r"DIV R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                let number = vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()];
                if number == 0 {
                    println!("Error: It is not possible to divide by zero.");
                    continue;
                }
                vec[0] = (vec[0] / number) % (u + 1);
            } else {
                println!("Error: Wrong syntax on DIV Ri.");
                continue;
            }
        } else if command.starts_with("MOD") {
            let re = Regex::new(r"MOD R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[0] = (vec[0] % vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()])
                    % (u + 1);
            } else {
                println!("Error: Wrong syntax on MOD Ri.");
                continue;
            }
        } else if command.starts_with("AND") {
            let re = Regex::new(r"AND R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[0] = (vec[0] & vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()])
                    % (u + 1);
            } else {
                println!("Error: Wrong syntax on AND Ri.");
                continue;
            }
        } else if command.starts_with("OR") {
            let re = Regex::new(r"OR R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[0] = (vec[0] | vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()])
                    % (u + 1);
            } else {
                println!("Error: Wrong syntax on OR Ri.");
                continue;
            }
        } else if command.starts_with("XOR") {
            let re = Regex::new(r"XOR R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[0] = (vec[0] ^ vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()])
                    % (u + 1);
            } else {
                println!("Error: Wrong syntax on XOR Ri.");
                continue;
            }
        } else if command.starts_with("LOAD") {
            let re = Regex::new(r"LOAD R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[0] = vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()];
            } else {
                println!("Error: Wrong syntax on LOAD Ri.");
                continue;
            }
        } else if command.starts_with("STORE") {
            let re = Regex::new(r"STORE R(\d+)").unwrap();
            if re.is_match(&command) {
                let caps = re.captures(&command).unwrap();
                vec[caps.get(1).unwrap().as_str().parse::<usize>().unwrap()] = vec[0];
            } else {
                println!("Error: Wrong syntax on STORE Ri.");
                continue;
            }
        } else {
            println!("Error: {} is no valid command.", command);
            continue;
        }
        for i in 0..(u + 1) {
            let register_name = format!("R{}", i);
            // Since u is the highest number, we use its length to calculate the paddings.
            let length1 = u.to_string().len();
            let length2 = length1 + 1;
            if i % 8 == 0 && i > 0 {
                println!("|");
            }
            print!(
                "| {:>length2$} | {:>length1$} ",
                register_name,
                vec[i as usize],
                length1 = length1,
                length2 = length2
            );
        }
        print!("|\n");
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error: Can't read input.");
    input.trim().to_string()
}
