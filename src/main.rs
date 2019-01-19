pub mod machine;

use crate::machine::interface::Machine;
fn get_trait() -> Machine <'static>{
    let m = machine::x64::new();
    return m;
}

fn main() {
    let m = get_trait();
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(n) => {
                let result = m.asm(input.to_string(),0);
                match result {
                    Ok(r) => {
                        m.write_instruction(r.bytes);
                        m.print_register();
                        m.print_stack();
                    }
                    Err(e) => println!("failed to assemble, err: {:?}", e),
                }
            }
            Err(error) => println!("error when read your input: {}", error),
        }
        println!();
    }
}
