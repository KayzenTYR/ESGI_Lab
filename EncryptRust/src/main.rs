mod Interaction_User;
mod File_Manager;
mod Encryption;
mod Decryption;
mod Common_Crypt;
mod Utils;

use std::process;

use Interaction_User::ask_path_to_file;

fn main() {
    Interaction_User::display_single_msg("Welcome to the Encryption/Description Program");

    let (target_path, target_content) = ask_path_to_file("file");

    if !target_content.is_empty() {
    
        let choose = Interaction_User::select_option(
            &[
                (1, "Encrypt a file or message"),
                (2, "Decrypt a file or message"),
                (3, "Exit")
            ],
            "What do you want to do ?",
            3,
        );

        match choose.as_str() {
            "Encrypt a file or message" => Encryption::start(Utils::remove_extension(&target_path), &target_content),
            "Decrypt a file or message" => Decryption::start(Utils::remove_extension(&target_path), &target_content),
            "Exit" => {},
            _ => {
                Interaction_User::display_single_msg("Not supporte any other option");
            } 
        }
    }

    process::exit(0)

}
