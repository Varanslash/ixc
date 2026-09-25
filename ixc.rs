fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read(args[1].clone()).expect("Couldn't read code");
    let mut tape = vec![0u32; 65535];
    let mut instrdem: u8 = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < code.len() {
        match code[i] {
            0x00 => {
                if instrdem == 10 {
                    instrdem = 0
                }
                else {
                    instrdem += 1
                }
                i += 1
            }
            0x01 => {
                match instrdem {
                    1 => {
                        j += 1;
                        i += 1;
                    }

                    2 => {
                        j -= 1;
                        i += 1;
                    }

                    3 => {
                        tape[j] += 1;
                        i += 1;
                    }

                    4 => {
                        tape[j] -= 1;
                        i += 1;
                    }

                    5 => {
                        if tape[j] > 0 {
                            i = tape[j+1] as usize;
                        }
                        else {
                            i += 1;
                        }
                    }

                    6 => {
                        print!("{}", tape[j] as u8 as char);
                        i += 1
                    }
                    _ => {
                        if instrdem < 11 {
                            i += 1;
                        }
                        else {
                            eprintln!("Error: Overflow in instruction determinator");
                            std::process::exit(1);
                        }
                    }
                }
            }
            _ => {
                eprintln!("Error: Unknown byte in code");
                std::process::exit(1);
            }
        }
    }
}