use std::{
    io::{self, Write},
    path::PathBuf,
    fs,
};

fn get_input(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    Ok(response.trim().to_string())
}

struct FileEditor {
    file_path: PathBuf,
    file_content: Vec<String>,
}

impl FileEditor {
    fn new(file_path: PathBuf) -> Self {
        let mut obj = Self { file_path, file_content: Vec::new() };
        if let Err(err) = obj.load_file_contents() {
            eprintln!("\nAn error occured when loading file contents: {err}");
            std::process::exit(0);
        }

        obj
    }

    fn prompt_create_file(file_path: PathBuf) -> io::Result<Option<PathBuf>> {
        loop {
            let response = get_input("Create new file (y/n)? ");
            match response?.to_lowercase().as_str() {
                "n" => return Ok(None),
                "y" =>{
                    let _ = fs::File::create(&file_path)?;
                    println!("'{}' created successfully", file_path.display());
                    return Ok(Some(file_path))
                }
                _ => {
                    println!("\nPlease enter a valid option");
                    continue;
                }
            }
        }
    }

    fn load_file_contents(&mut self) -> io::Result<()> {
        let file_content = fs::read_to_string(&self.file_path)?;

        let file_contents_as_list: Vec<String> = file_content.split("\n")
            .map(|x| x.to_string())
            .collect();
        self.file_content = file_contents_as_list;

        Ok(())
    }

    fn get_file_contents(&self) -> &Vec<String> {
        &self.file_content
    }

    fn view(&mut self) -> io::Result<()> {
        let times = 40;

        println!("\n{}", "=".repeat(times));
        for (index, value) in self.get_file_contents().iter().enumerate() {
            println!("{}  {}", index+1, value);
        }
        println!("{}\n", "=".repeat(times));

        Ok(())
    }

    fn add_line(&mut self) -> io::Result<()> {
        let response = get_input("Enter line: ")?;
        self.file_content.push(response);

        println!("\nLine successfully Added\n");
        Ok(())
    }

    fn edit_line(&mut self) -> io::Result<()> {
        loop {
            if let Ok(index) = get_input("Enter line number to edit: ")?.parse::<usize>() {
                if index-1 <= self.get_file_contents().len() {
                    let response = get_input("New Content: ")?;
                    self.file_content[index-1] = response;
                    println!("\nLine successfully edited\n");
                    break;
                } else {
                    println!("Please enter a valid line number");
                }
            } else {
                println!("\nPlease enter a number");
            }
        }
        Ok(())
    }

    fn remove_line(&mut self) -> io::Result<()> {
        loop {
            if let Ok(index) = get_input("Enter line number to remove: ")?.parse::<usize>() {
                if index-1 <= self.get_file_contents().len() {
                    self.file_content.remove(index-1);
                    println!("\nLine successfully removed\n");
                    break;
                } else {
                    println!("Please enter a valid line number");
                }
            } else {
                println!("\nPlease enter a number");
            }
        }
        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        fs::write(&self.file_path, self.file_content.join("\n"))?;
        println!("\nFile successfully saved\n");
        Ok(())
    }
}

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
            println!("1. View File Contents\n2. Add line\n3. Edit line\n4. Delete line\n5. Save\n6. Quit");
            let response = get_input("What do u wanna do? ");

            let response = match response?.parse::<u8>() {
                Ok(val) => val,
                Err(_) => {
                    println!("\nPlease enter a number");
                    continue;
                }
            };
            match response {
                val @ 1..=6 => {
                    match val {
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
                                eprintln!("An error occurred when removing line: {err}");
                            }
                        }
                        _ => ()
                    }
                }

                _ => {
                    println!("\nPlease Select a valid option");
                }
            }
        }
    }

    Ok(())
}
