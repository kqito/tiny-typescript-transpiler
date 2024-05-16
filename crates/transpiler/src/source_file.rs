use ast::statement::*;
use ast::*;
use std::{fs::read_to_string, path::Path};

#[derive(Debug, PartialEq, Clone)]
pub struct SourceFile {
    pub name: String,
    pub src: String,
    pub path: String,
    pub node: Option<Node<Modules>>,
}

pub enum SourceFileError {
    FailedReadFile,
}

impl SourceFile {
    pub fn load(path: &str) -> Result<Self, SourceFileError> {
        let name = match Path::new(path).file_name().unwrap().to_str() {
            Some(name) => String::from(name),
            None => return Err(SourceFileError::FailedReadFile),
        };

        let src = match read_to_string(path) {
            Ok(src) => src,
            Err(_error) => return Err(SourceFileError::FailedReadFile),
        };

        Ok(Self {
            name,
            src,
            path: String::from(path),
            node: None,
        })
    }

    pub fn set_module(&mut self, modules: Modules) {
        self.node = Some(Node::new(Loc::new(0, 0), modules));
    }
}
