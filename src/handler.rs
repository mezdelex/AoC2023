use std::{
    fs::File,
    io::{Error, Read},
};

pub struct Handler<'a> {
    path_to_file: &'a str,
}

impl<'a> Handler<'a> {
    pub fn new(path_to_file: &'a str) -> Self {
        Self { path_to_file }
    }

    pub fn handle_input(&self) -> Result<String, Error> {
        let mut input_file = File::open(self.path_to_file)?;
        let mut input_data = String::new();
        input_file.read_to_string(&mut input_data)?;

        Ok(input_data)
    }
}
