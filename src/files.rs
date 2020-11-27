use std::io;
use std::path::Path;
use std::{
    fs::{self, File},
    io::Write,
};

pub fn create_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    let mut is_parent_path_exist = false;

    if let Some(parent) = path.as_ref().parent() {
        if parent.exists() {
            is_parent_path_exist = true
        }

        if !is_parent_path_exist {
            fs::create_dir_all(parent)?;
        }
    }

    return fs::File::create(path);
}

pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut f = create_file(path)?;
    f.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test_files {

    #[test]
    fn test_write_file() {
        use super::*;

        crate::_log::init();
        let test_file_path = std::path::Path::new("/tmp/test/test");

        let test_file_content = String::from("test content\n");

        log::debug!(
            "write file<{}> to path<{:?}>",
            test_file_content,
            test_file_path
        );

        match write_file(test_file_path, &test_file_content) {
            Ok(_) => {}
            Err(e) => {
                log::error!("{}", e);
                assert_eq!(e.to_string(), "");
            }
        }
    }
}
