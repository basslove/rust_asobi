use std::fmt;

pub trait Entry {
    fn get_name(&self) -> String;
    fn get_size(&self) -> u32;
    fn print_list(&self, prefix: &str);
    fn print(&self) {
        self.print_list("");
    }
}

#[derive(Clone)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: String, size: u32) -> Self {
        Self { name, size }
    }
}

impl Entry for File {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> u32 {
        self.size
    }

    fn print_list(&self, prefix: &str) {
        println!("{} / {}", prefix, self)
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.get_name(), self.get_size())
    }
}

struct Directory {
    name: String,
    directory: Vec<Box<dyn Entry>>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory { name, directory: Vec::new() }
    }

    fn add(&mut self, entry: Box<dyn Entry>) {
        self.directory.push(entry);
    }
}

impl Entry for Directory {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_size(&self) -> u32 {
        let mut size = 0;
        for entry in &self.directory {
            size += entry.get_size()
        }
        size
    }

    fn print_list(&self, prefix: &str) {
        println!("{}/{}", prefix, self);

        for entry in &self.directory {
            entry.print_list(format!("{}/{}", prefix, self.name).as_str());
        }
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.get_name(), self.get_size())
    }
}

pub fn execute() {
    println!("composite");

    let mut root_dir = Box::new(Directory::new("root".to_string()));
    let mut bin_dir = Box::new(Directory::new("bin".to_string()));
    let tmp_dir = Box::new(Directory::new("tmp".to_string()));
    let mut usr_dir = Box::new(Directory::new("usr".to_string()));

    bin_dir.add(Box::new(File::new("vi".to_string(), 10000)));
    bin_dir.add(Box::new(File::new("emacs".to_string(), 20000)));

    root_dir.add(bin_dir);
    root_dir.add(tmp_dir);

    root_dir.print();

    println!();

    let mut yuki = Box::new(Directory::new("yuki".to_string()));
    yuki.add(Box::new(File::new("diary.html".to_string(), 100)));
    yuki.add(Box::new(File::new("Composite.java".to_string(), 200)));

    let mut hanako = Box::new(Directory::new("hanako".to_string()));
    hanako.add(Box::new(File::new("memo.tex".to_string(), 300)));

    let mut tomura = Box::new(Directory::new("tomura".to_string()));
    tomura.add(Box::new(File::new("game.doc".to_string(), 400)));
    tomura.add(Box::new(File::new("jumk.mail".to_string(), 500)));

    usr_dir.add(yuki);
    usr_dir.add(hanako);
    usr_dir.add(tomura);

    root_dir.add(usr_dir);

    root_dir.print();
}
