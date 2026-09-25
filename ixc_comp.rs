fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read_to_string(args[1].clone()).expect("Couldn't read file");
    let mut output: Vec<u8> = Vec::new();
    
    for bit in code.chars() {
        match bit {
            '0' => {
                output.push(0x00)
            }
            '1' => {
                output.push(0x01)
            }
            _ => {
                eprintln!("Error: Unknown value found; only use 0 and 1 in code");
                std::process::exit(1);
            }
        }
    }
    
    let _ = std::fs::write(args[2].clone(), output);
}