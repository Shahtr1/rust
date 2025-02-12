#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Folder {
            name,
            contents: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        self.contents.push(File { name });
    }

    fn delete_file(&mut self, index: usize) -> File {
        let option = self.get_file(index);
        match option {
            Option::Some(_value) => self.contents.remove(index),
            Option::None => File {
                name: "".to_string(),
            },
        }
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

pub fn run() {
    let mut new_folder = Folder::new("New Folder".to_string());
    new_folder.create_file("name1.txt".to_string());
    new_folder.create_file("name2.txt".to_string());

    println!("{:#?}", new_folder);

    new_folder.delete_file(0);
    println!("{:#?}", new_folder);

    new_folder.delete_file(10);

    let file = new_folder.get_file(0);

    match file {
        Option::Some(value) => println!("File is {value:?}"),
        Option::None => println!("There is no file"),
    }
}
