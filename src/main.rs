use std::{io, path::PathBuf};
use text_editor::{FileEditor, get_input};

fn main() -> io::Result<()> {
    println!("Welcome to the Text Editor\n");

    loop {
        let mut editor: FileEditor;
        let response = match get_input("Enter filename to open. 'q' to quit: ") {
            Err(err) => {
                println!("An error occurred getting the filename: {err}");
                break;
            }
            Ok(filename) => filename,
        };

        // Responsible for Quitting
        if response.to_lowercase().as_str() == "q" {
            println!("Goodbye for now :)");
            std::process::exit(0);
        }

        if PathBuf::from(&response).exists() {
            editor = FileEditor::new(PathBuf::from(&response));
        } else {
            match FileEditor::prompt_create_file(PathBuf::from(&response)) {
                Ok(Some(value)) => {
                    editor = FileEditor::new(value);
                }
                Ok(None) => {
                    println!("Goodbye for now:)");
                    std::process::exit(0);
                }
                Err(err) => {
                    eprintln!("An error occured: {err}");
                    std::process::exit(1);
                }
            }
        }

        // MAIN LOGIC OF THE FILE
        loop {
            println!(
                "1. View File Contents\n2. Add line\n3. Edit line\n4. Delete line\n5. Save\n6. Quit"
            );
            let response = get_input("What do u wanna do? ");

            let response = match response?.parse::<u8>() {
                Ok(val) => val,
                Err(_) => {
                    println!("\nPlease enter a number");
                    continue;
                }
            };
            match response {
                val @ 1..=6 => match val {
                    6 => {
                        println!("Goodbye for now :)");
                        std::process::exit(0);
                    }
                    1 => {
                        if let Err(err) = editor.view() {
                            eprintln!("An error occurred when viewing file contents: {err}");
                        }
                    }
                    2 => {
                        if let Err(err) = editor.add_line() {
                            eprintln!("An error occurred when adding line: {err}");
                        }
                    }
                    3 => {
                        if let Err(err) = editor.edit_line() {
                            eprintln!("An error occurred when editing line: {err}");
                        }
                    }
                    4 => {
                        if let Err(err) = editor.remove_line() {
                            eprintln!("An error occurred when removing line: {err}");
                        }
                    }
                    5 => {
                        if let Err(err) = editor.save() {
                            eprintln!("An error occurred when saving file: {err}");
                        }
                    }
                    _ => (),
                },

                _ => {
                    println!("\nPlease Select a valid option");
                }
            }
        }
    }

    Ok(())
}
