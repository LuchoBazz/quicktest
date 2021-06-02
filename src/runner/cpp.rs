use std::path::PathBuf;

pub struct CppRunner {
    file_name: PathBuf,
}

impl CppRunner {

    pub fn new(file_name: PathBuf) -> CppRunner {
        CppRunner {
            file_name: file_name
        }
    }
    
    #[cfg(unix)]
    pub fn compile(&self) -> &str {
        println!("Unix");
        self.file_name.to_str().unwrap()
    }

    #[cfg(windows)]
    pub fn compile(&self) -> &str {
        println!("Windows");
        self.file_name.to_str().unwrap()
    }
}