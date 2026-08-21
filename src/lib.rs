use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

pub fn get_input(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut response = String::new();
    io::stdin().read_line(&mut response)?;

    Ok(response.trim().to_string())
}

pub struct FileEditor {
    file_path: PathBuf,
    file_content: Vec<String>,
}

impl FileEditor {
    pub fn new(file_path: PathBuf) -> Self {
        let mut obj = Self {
            file_path,
            file_content: Vec::new(),
        };
        if let Err(err) = obj.load_file_contents() {
            eprintln!("\nAn error occured when loading file contents: {err}");
            std::process::exit(1);
        }

        obj
    }

    pub fn prompt_create_file(file_path: PathBuf) -> io::Result<Option<PathBuf>> {
        loop {
            let response = get_input("Create new file (y/n)? ");
            match response?.to_lowercase().as_str() {
                "n" => return Ok(None),
                "y" => {
                    let _ = fs::File::create(&file_path)?;
                    println!("'{}' created successfully", file_path.display());
                    return Ok(Some(file_path));
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

        let file_contents_as_list: Vec<String> =
            file_content.split("\n").map(|x| x.to_string()).collect();
        self.file_content = file_contents_as_list;

        Ok(())
    }

    pub fn get_file_contents(&self) -> &[String] {
        &self.file_content
    }

    pub fn view(&self) -> io::Result<()> {
        let times = 40;

        println!("\n{}", "=".repeat(times));
        for (index, value) in self.get_file_contents().iter().enumerate() {
            println!("{}  {}", index + 1, value);
        }
        println!("{}\n", "=".repeat(times));

        Ok(())
    }

    fn valid_line_number(&self, index: usize) -> bool {
        index >= 1 && index <= self.get_file_contents().len()
    }

    pub fn add_line(&mut self) -> io::Result<()> {
        let response = get_input("Enter line: ")?;
        self.file_content.push(response);

        println!("\nLine successfully Added\n");
        Ok(())
    }

    pub fn edit_line(&mut self) -> io::Result<()> {
        loop {
            if let Ok(index) = get_input("Enter line number to edit: ")?.parse::<usize>() {
                if !self.valid_line_number(index) {
                    println!("Please enter a valid line number");
                    continue;
                }
                let response = get_input("New Content: ")?;
                self.file_content[index - 1] = response;
                println!("\nLine successfully edited\n");
                break;
            } else {
                println!("\nPlease enter a number");
            }
        }
        Ok(())
    }

    pub fn remove_line(&mut self) -> io::Result<()> {
        loop {
            if let Ok(index) = get_input("Enter line number to remove: ")?.parse::<usize>() {
                if !self.valid_line_number(index) {
                    println!("Please enter a valid line number");
                    continue;
                }
                self.file_content.remove(index - 1);
                println!("\nLine successfully removed\n");
                break;
            } else {
                println!("\nPlease enter a number");
            }
        }
        Ok(())
    }

    pub fn save(&self) -> io::Result<()> {
        fs::write(&self.file_path, self.file_content.join("\n"))?;
        println!("\nFile successfully saved\n");
        Ok(())
    }
}
