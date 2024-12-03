mod Interaction_User;
mod File_Manager;
mod Encryption;
mod Decryption;
mod Utils;

use std::process;

fn main() {
    let mut error_nb = 0;
    let mut target_path;
    let mut target_content;
    Interaction_User::display_single_msg("Welcome to the Encryption/Description Program");
    Interaction_User::display_single_msg("Choose the file that will be process");

    loop {

        target_path = Interaction_User::pick_up_input("Input the path :");
    
        target_content = File_Manager::get_content(&target_path);
        
        if target_content.is_empty() {
            error_nb += 1
        }  else {
            break
        }
        
        if error_nb == 3 {
            break
        }
    }

    if !target_content.is_empty() {

        let msgs = [
            "",
            "Choose an option : ",
            "1. Encrypt a file or message",
            "2. Decrypt a file or message",
            "3. Exit"
        ];
        Interaction_User::display_severage_msgs(&msgs);
    
        error_nb = 0;
    
        loop {
            let error_nb_clone = error_nb.clone();

            match Interaction_User::pick_up_input_i32("Enter a option here :") {
                1 => Encryption::start(Utils::remove_extension(&target_path), &target_content),
                2 => Decryption::start(),
                3 => break,
                _ => {
                    Interaction_User::display_single_msg("Wrong option ...");
                    error_nb +=1;
                } 
            }
    
            if error_nb == 3 {
                Interaction_User::display_single_msg("To much errors, exiting process ...");
                break;
            } else if error_nb == error_nb_clone {
                break;
            }
    
        }
    }

    process::exit(0)

}
