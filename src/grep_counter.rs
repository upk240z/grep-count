use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use regex::Regex;

pub struct GrepCounter {
    re: Regex,
    dict: HashMap<String, u32>,
    files: Vec<String>,
}

impl GrepCounter {
    pub fn new(expresssion: &String, files: Vec<String>) -> Self {
        let Ok(re) = Regex::new(expresssion) else {
            eprintln!("regular expression error");
            process::exit(0);
        };
        let dict: HashMap<String, u32> = HashMap::new();
        Self {
            re,
            dict,
            files,
        }
    }

    fn scan_file(&mut self, index: usize) {
        let Some(path) = self.files.get(index) else {
            return
        };

        let Ok(file) = File::open(path) else {
            eprintln!("file open error: {}", path);
            process::exit(0);
        };

        let reader = BufReader::new(file);
        let mut stream = reader.split(0x0a);

        print!("Scanning {} ... ", path);

        while let Some(result) = stream.next() {
            let Ok(bytes) = result else {
                eprintln!("read bytes error");
                continue;
            };

            let Ok(line) = String::from_utf8(bytes) else {
                eprintln!("read line error");
                continue;
            };

            let Some(cap) = self.re.captures(line.as_str()) else {
                continue;
            };

            if cap.len() >= 2 {
                let Some(m) = cap.get(1) else {
                    eprintln!("capture error: {}", line);
                    continue;
                };
                let key = m.as_str().to_string();
                let current = self.dict.get(&key).unwrap_or(&0);
                self.dict.insert(key, current + 1);
            }
        }

        println!("OK");
    }

    pub fn scan(&mut self) -> &HashMap<String, u32> {
        for index in 0..self.files.len() {
            self.scan_file(index);
        }

        &self.dict
    }
}
