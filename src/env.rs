use std::env;

pub fn get_current_dir() -> Option<String> {
    match env::current_dir() {
        Ok(dir) => {
            let dir_str: String = dir.to_string_lossy().into_owned();
            Some(dir_str)
        },
        Err(_) => None,
    }
}

pub fn get_prompt_symbol() -> char {
    unsafe {
        if libc::geteuid() == 0 {
            return '#'; 
        }
    }
    '$'
}

