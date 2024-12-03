use std::io::{self, Read, Write};

pub fn display_single_msg(msg: &str) {
    println!("{}", msg);

    return;
}

pub fn display_severage_msgs(msgs: &[&str]) {
    for msg in msgs {
        display_single_msg(msg);
    }

    return;
}

pub fn pick_up_input(msg: &str) -> String {
    let mut input = String::new();

    println!("{}", msg);

    let _ = io::stdout().flush();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {}
    };

    return input.trim().to_string();
}


pub fn pick_up_inputs(msgs: &[&str]) -> Vec<String> {
    if msgs.len() == 0 {
        // throw an error
    }
    let mut inputs = Vec::new();
    let mut input;
    let mut index = 0;
    let mut msg;

    loop {
        print!("Enter input (or type 'exit' to finish): ");
        if msgs.len() == 1 {
            msg = msgs[0];
        } else {
            msg = msgs[index];
        }
        
        input = pick_up_input(msg);
        
        if input.eq_ignore_ascii_case("exit") || input.is_empty() {
            break;
        }

        inputs.push(input);

        index = (index + 1) % msgs.len();
    }
    
    return inputs;
}

pub fn pick_up_input_i32(msg: &str) -> i32 {
    return match pick_up_input(msg).parse::<i32>() {
        Ok(r) => r,
        Err(_) => -1
    };
}
