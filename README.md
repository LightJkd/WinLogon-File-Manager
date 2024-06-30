# WinLogon File Manager

WinLogon File Manager is a command-line utility for managing the WinLogon registry key on Windows. This program allows you to add and remove file paths to the `Userinit` value, controlling startup applications. **Administrator privileges are required to run this tool.**

## Features

- Add a file path to the WinLogon `Userinit` registry key.
- Remove a file path from the WinLogon `Userinit` registry key.
- Simple and intuitive command-line interface.

## Prerequisites

- Rust programming language installed
- Administrator privileges

## How to Build

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/WinLogonFileManager.git
    cd WinLogonFileManager
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the executable:
    ```sh
    cargo run --release
    ```

## How It Works

1. **Ensure Administrator Privileges:**
    The program first checks if it is running with administrator privileges. If not, it prompts the user to restart with the necessary permissions.

2. **User Menu:**
    The main menu allows users to choose between adding a file path, removing a file path, or exiting the program.

3. **Adding a File Path:**
    - Prompts the user to input the file path.
    - Adds the provided path to the `Userinit` value in the WinLogon registry key.

4. **Removing a File Path:**
    - Prompts the user to input the file path to be removed.
    - Removes the specified path from the `Userinit` value in the WinLogon registry key, if it exists.

5. **Post-Action Menu:**
    - Users can choose to reboot the system to apply changes or exit the program.

## Commands

- **Add a file path to WinLogon:**
    ```sh
    Choose an option:
    (1) Add a file to WinLogon
    ```

- **Remove a file path from WinLogon:**
    ```sh
    Choose an option:
    (2) Delete a file from WinLogon
    ```

- **Exit the program:**
    ```sh
    Choose an option:
    (3) Exit
    ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
