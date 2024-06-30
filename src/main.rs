use std::io::{self, Write};
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;
use winapi::um::winnt::TOKEN_ELEVATION;

mod winlogon_manager {
    use super::*;

    pub struct WinLogonManager;

    impl WinLogonManager {
        pub fn add_to_winlogon(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let winlogon_key = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon", KEY_ALL_ACCESS)?;

            let existing_value: String = winlogon_key.get_value("Userinit")?;
            let new_value = format!("{},{}", existing_value, file_path);
            winlogon_key.set_value("Userinit", &new_value)?;

            Ok(())
        }

        pub fn delete_from_winlogon(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let winlogon_key = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon", KEY_ALL_ACCESS)?;

            let existing_value: String = winlogon_key.get_value("Userinit")?;
            println!("Current Userinit value: {}", existing_value);

            let mut values: Vec<&str> = existing_value.split(',').collect();
            let initial_length = values.len();

            values.retain(|&s| s.trim() != file_path);

            if values.len() == initial_length {
                println!("File not found in the current Userinit value.");
            } else {
                println!("File found and removed from the current Userinit value.");
            }

            if values.is_empty() {
                println!("Userinit value cannot be empty. No changes were made.");
            } else {
                let new_value = values.join(",");
                winlogon_key.set_value("Userinit", &new_value)?;
                println!("Updated Userinit value: {}", new_value);
            }

            Ok(())
        }
    }
}

mod utils {
    use super::*;

    pub fn is_elevated() -> bool {
        unsafe {
            let mut token: winapi::um::winnt::HANDLE = std::ptr::null_mut();
            if winapi::um::processthreadsapi::OpenProcessToken(
                winapi::um::processthreadsapi::GetCurrentProcess(),
                winapi::um::winnt::TOKEN_QUERY,
                &mut token,
            ) == 0
            {
                return false;
            }

            let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
            let mut ret_len = 0;

            let success = winapi::um::securitybaseapi::GetTokenInformation(
                token,
                winapi::um::winnt::TokenElevation,
                &mut elevation as *mut _ as *mut _,
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut ret_len,
            ) != 0;

            winapi::um::handleapi::CloseHandle(token);

            success && elevation.TokenIsElevated != 0
        }
    }

    pub fn press_any_key_to_close() {
        println!("Press any key to close the program.");
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    }

    pub fn post_action_menu() {
        println!("Choose an option:");
        println!("(1) Reboot to check");
        println!("(2) Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Rebooting the system...");
                Command::new("shutdown")
                    .arg("/r")
                    .arg("/t")
                    .arg("0")
                    .output()
                    .expect("Failed to execute reboot command");
            },
            "2" => {
                println!("Exiting the program. Press any key to close.");
                io::stdin().read_line(&mut String::new()).expect("Failed to read line");
            },
            _ => {
                println!("Invalid option. Exiting the program. Press any key to close.");
                io::stdin().read_line(&mut String::new()).expect("Failed to read line");
            }
        }
    }
}

fn main() {
    if !utils::is_elevated() {
        println!("This program requires administrator privileges.");
        utils::press_any_key_to_close();
        return;
    }

    loop {
        println!("Choose an option:");
        println!("(1) Add a file to WinLogon");
        println!("(2) Delete a file from WinLogon");
        println!("(3) Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Please enter the path to the file to add to WinLogon:");
                let mut file_path = String::new();
                io::stdin().read_line(&mut file_path).expect("Failed to read line");
                let file_path = file_path.trim();

                match winlogon_manager::WinLogonManager::add_to_winlogon(file_path) {
                    Ok(_) => println!("The file was successfully added to WinLogon."),
                    Err(e) => println!("Failed to add the file to WinLogon. Error: {:?}", e),
                }
                utils::post_action_menu();
            },
            "2" => {
                println!("Please enter the path to the file to delete from WinLogon:");
                let mut file_path = String::new();
                io::stdin().read_line(&mut file_path).expect("Failed to read line");
                let file_path = file_path.trim();

                match winlogon_manager::WinLogonManager::delete_from_winlogon(file_path) {
                    Ok(_) => println!("The file was successfully deleted from WinLogon."),
                    Err(e) => println!("Failed to delete the file from WinLogon. Error: {:?}", e),
                }
                utils::press_any_key_to_close();
            },
            "3" => {
                println!("Exiting the program. Press any key to close.");
                io::stdin().read_line(&mut String::new()).expect("Failed to read line");
                break;
            },
            _ => println!("Invalid option. Please choose again."),
        }
    }
}
