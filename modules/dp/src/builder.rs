use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub trait Builder {
    fn make_title(&mut self, title: &str);
    fn make_string(&mut self, s: &str);
    fn make_items(&mut self, items: Vec<String>);
    fn close(&mut self);
}

#[derive(Clone)]
struct TextBuilder {
    string_builder: Vec<String>,
}

impl TextBuilder {
    fn new() -> Self {
        Self { string_builder: Vec::new() }
    }

    fn get_text_string(&self) -> String {
        self.string_builder.join("")
    }
}

impl Builder for TextBuilder {
    fn make_title(&mut self, title: &str) {
        self.string_builder.push("============================".to_string());
        self.string_builder.push("『".to_string());
        self.string_builder.push(title.to_string());
        self.string_builder.push("』".to_string());
    }

    fn make_string(&mut self, s: &str) {
        self.string_builder.push("■".to_string());
        self.string_builder.push(s.to_string());
        self.string_builder.push("\n\n".to_string());
    }

    fn make_items(&mut self, items: Vec<String>) {
        for s in items {
            self.string_builder.push("・".to_string());
            self.string_builder.push(s.to_string());
            self.string_builder.push("\n".to_string());
        }
        self.string_builder.push("\n".to_string());
    }

    fn close(&mut self) {
        self.string_builder.push("============================".to_string());
    }
}

#[derive(Clone)]
struct HtmlBuilder {
    file_name: String,
    string_builder: Vec<String>,
}

impl HtmlBuilder {
    fn new() -> Self {
        Self {
            string_builder: Vec::new(),
            file_name: "untitle.html".to_string(),
        }
    }

    fn get_html_result(&self) -> String {
        self.file_name.to_string()
    }
}

impl Builder for HtmlBuilder {
    fn make_title(&mut self, title: &str) {
        self.string_builder.push("<!DOCTYPE html>\n".to_string());
        self.string_builder.push("<html>\n".to_string());
        self.string_builder.push("<head><title>\n".to_string());
        self.string_builder.push(title.to_string());
        self.string_builder.push("</title></head>\n".to_string());
        self.string_builder.push("<body>\n".to_string());
        self.string_builder.push("<h1>\n".to_string());
        self.string_builder.push(title.to_string());
        self.string_builder.push("</h1>\n\n".to_string());
    }

    fn make_string(&mut self, s: &str) {
        self.string_builder.push("<p>".to_string());
        self.string_builder.push(s.to_string());
        self.string_builder.push("</p>\n\n".to_string());
    }

    fn make_items(&mut self, items: Vec<String>) {
        self.string_builder.push("<ul>".to_string());
        for s in items {
            self.string_builder.push("<li>".to_string());
            self.string_builder.push(s);
            self.string_builder.push("</li>\n".to_string());
        }
        self.string_builder.push("</ul>\n\n".to_string());
    }

    fn close(&mut self) {
        self.string_builder.push("</body>".to_string());
        self.string_builder.push("</html>\n".to_string());

        let bind = self.file_name.clone();
        let path = Path::new(bind.as_str());
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(self.string_builder.join("").as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

struct Director {}

impl Director {
    fn new() -> Self {
        Self {}
    }

    fn construct(&self, builder: &mut impl Builder) {
        builder.make_title("greeting");
        builder.make_string("一般的挨拶");
        builder.make_items(vec!["hello".to_string(), "hi.".to_string()]);
        builder.make_string("時間帯挨拶");
        builder.make_items(vec!["good morning".to_string(), "good afternoon".to_string(), "good evening".to_string()]);
        builder.close();
    }
}

pub fn execute() {
    println!("builder");

    let moji = "html";

    if moji == "text" {
        let mut text_builder = TextBuilder::new();
        let director = Director::new();
        director.construct(&mut text_builder);
        let result = text_builder.get_text_string();
        println!("{}", result);
    }

    if moji == "html" {
        let mut html_builder = HtmlBuilder::new();
        let director = Director::new();
        director.construct(&mut html_builder);
        let file_name = html_builder.get_html_result();
        println!("{}", file_name);
    }
}
