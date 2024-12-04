use std::io::{self, Write};
use crate::File_Manager;

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

    print!("{}", msg);

    let _ = io::stdout().flush();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {}
    };

    return input.trim().to_string();
}


// pub fn pick_up_inputs(msgs: &[&str]) -> Vec<String> {
//     if msgs.len() == 0 {
//         // throw an error
//     }
//     let mut inputs = Vec::new();
//     let mut input;
//     let mut index = 0;
//     let mut msg;

//     loop {
//         print!("Enter input (or type 'exit' to finish): ");
//         if msgs.len() == 1 {
//             msg = msgs[0];
//         } else {
//             msg = msgs[index];
//         }
        
//         input = pick_up_input(msg);
        
//         if input.eq_ignore_ascii_case("exit") || input.is_empty() {
//             break;
//         }

//         inputs.push(input);

//         index = (index + 1) % msgs.len();
//     }
    
//     return inputs;
// }

pub fn pick_up_input_i32(msg: &str) -> i32 {
    return match pick_up_input(msg).parse::<i32>() {
        Ok(r) => r,
        Err(_) => -1
    };
}

pub fn select_option(options: &[(i32, &str)], prompt: &str, error_limit: usize) -> String {
    display_severage_msgs(&[
        prompt,
        "Choose an option:",
        &options
            .iter()
            .map(|(opt, size)| format!("{}. {}", opt, size))
            .collect::<Vec<_>>()
            .join("\n"),
    ]);

    let mut choose = "";
    let mut error_count = 0;

    loop {
        let user_input = pick_up_input_i32("Enter an option here:");

        if let Some(&(_, option)) = options.iter().find(|&&(opt, _)| opt == user_input) {
            choose = option;
            break;
        } else {
            display_single_msg("Wrong option...");
            error_count += 1;
        }

        if error_count >= error_limit {
            display_single_msg("Too many errors, exiting process...");
            break;
        }
    }

    return choose.to_string();
}

pub fn ask_path_to_file (msg: &str) -> (String, Vec<u8>) {
    let mut file_path ;
    let mut content;
    let mut error_nb = 0;

    display_single_msg(&format!("Choose the {} that will be process", msg));

    loop {

        file_path = pick_up_input("Input the path :");
    
        content = File_Manager::get_content(&file_path); 
        
        if content.is_empty() {
            error_nb += 1
        }  else {
            break
        }
        
        if error_nb == 3 {
            display_single_msg("To much errors, exiting process ...");
            break
        }
    }

    (file_path, content)
}
