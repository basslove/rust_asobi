trait Display {
    fn get_columns(&self) -> usize;
    fn get_rows(&self) -> usize;
    fn get_row_text(&self, row: usize) -> String;

    fn show(&self) {
        for i in 0..self.get_rows() {
            println!("{}", self.get_row_text(i));
        }
    }
}

struct StringDisplay {
    text: String,
}

impl StringDisplay {
    fn new(text: String) -> Self {
        Self { text }
    }
}

impl Display for StringDisplay {
    fn get_columns(&self) -> usize {
        let byte_length = self.text.len();
        let char_length = self.text.chars().count();

        if byte_length == char_length {
            self.text.len()
        } else {
            2 * self.text.chars().count()
        }
    }

    fn get_rows(&self) -> usize {
        1
    }

    fn get_row_text(&self, row: usize) -> String {
        if row == 0 {
            return self.text.clone();
        }

        "".to_string()
    }
}

struct SideBorder {
    display: Box<dyn Display>,
    border_char: char,
}

impl SideBorder {
    fn new(display: Box<dyn Display>, border_char: char) -> Self {
        Self { display, border_char }
    }
}

impl Display for SideBorder {
    fn get_columns(&self) -> usize {
        1 + self.display.get_columns() + 1
    }

    fn get_rows(&self) -> usize {
        self.display.get_rows()
    }

    fn get_row_text(&self, row: usize) -> String {
        format!("{}{}{}", self.border_char, self.display.get_row_text(row), self.border_char)
    }
}

struct FullBorder {
    display: Box<dyn Display>,
}

impl FullBorder {
    fn new(display: Box<dyn Display>) -> Self {
        Self { display }
    }

    fn make_line(&self, ch: char, count: usize) -> String {
        let mut buffer = "".to_string();
        for _ in 0..count {
            buffer.push(ch);
        }
        buffer
    }
}

impl Display for FullBorder {
    fn get_columns(&self) -> usize {
        1 + self.display.get_columns() + 1
    }

    fn get_rows(&self) -> usize {
        1 + self.display.get_rows() + 1
    }

    fn get_row_text(&self, row: usize) -> String {
        if row == 0 || row == self.display.get_rows() + 1 {
            format!("+{}+", self.make_line('-', self.display.get_columns()))
        } else {
            format!("|{}|", self.display.get_row_text(row - 1))
        }
    }
}

pub fn execute() {
    println!("decorator");

    let b1 = StringDisplay::new("hello  world.".to_string());
    b1.show();

    let b2 = SideBorder::new(Box::new(b1), '#');
    b2.show();

    let b3 = FullBorder::new(Box::new(b2));
    b3.show();

    let b4 = SideBorder::new(
        Box::new(FullBorder::new(Box::new(FullBorder::new(Box::new(SideBorder::new(
            Box::new(FullBorder::new(Box::new(StringDisplay::new("こんにちは。".to_string())))),
            '*',
        )))))),
        '/',
    );
    b4.show();
}
