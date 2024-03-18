const TAB_CHAR: u8 = 0x9;
const LF_CHAR: u8 = 0xa;
const CR_CHAR: u8 = 0xd;

pub trait FileContent {
    fn add_line_number(&mut self, skip_blank: bool);
    fn add_cr(&mut self);
    fn add_end_char(&mut self);
    fn add_tabs(&mut self);
    fn replace_byte(&mut self, from: u8, to: &str);
    fn squeeze_blank_lines(&mut self);
}

impl FileContent for String {
    fn add_line_number(&mut self, skip_blank: bool) {
        let mut tmp = String::new();
        let mut line_buffer = String::new();
        let mut count = 1;

        for byte in self.bytes() {
            line_buffer.push(byte as char);

            if byte == LF_CHAR {
                if line_buffer.len() == 1 && skip_blank {
                    tmp.push_str("\n")
                } else {
                    tmp.push_str(format!("{count}\t{line_buffer}").as_str());
                    count += 1
                }

                line_buffer.clear();
            }
        }

        if ! tmp.is_empty() {
            self.clear();
            self.push_str(tmp.as_str())
        } else {
            // no LF terminated strings
            if ! line_buffer.is_empty() {
                self.clear();
                self.push_str(format!("{count}\t{line_buffer}").as_str())
            }
        }
    }

    fn add_cr(&mut self) {
        self.replace_byte(CR_CHAR, "^M")
    }

    fn add_end_char(&mut self) {
        self.replace_byte(LF_CHAR, "$\n")
    }

    fn add_tabs(&mut self) {
        self.replace_byte(TAB_CHAR, "^I")
    }

    fn replace_byte(&mut self, from: u8, to: &str) {
        let mut tmp = String::new();

        for byte in self.bytes() {
            if byte != from {
                tmp.push(byte as char)
            } else {
                tmp.push_str(to)
            }
        }

        if ! tmp.is_empty() {
            self.clear();
            self.push_str(tmp.as_str())
        }
    }

    fn squeeze_blank_lines(&mut self) {
        let mut tmp = String::new();
        let mut line = String::new();
        let mut prior_blank_line = false;

        for byte in self.bytes() {
            if byte != LF_CHAR {
                line.push(byte as char);
                continue
            }

            if ! line.is_empty() {
                line.push(byte as char);
                tmp.push_str(line.as_str());
                line.clear();

                if prior_blank_line {
                    prior_blank_line = false
                }
            } else if ! prior_blank_line {
                line.push(byte as char);
                tmp.push_str(line.as_str());
                line.clear();
                prior_blank_line = true
            }
        }

        if ! tmp.is_empty() {
            self.clear();
            self.push_str(tmp.as_str())
        }
    }
}
