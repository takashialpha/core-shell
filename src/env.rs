use std::env;

pub fn get_current_dir() -> Option<String> {
    let current = match env::current_dir() {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => return None,
    };
    let home = env::var("HOME").unwrap_or_default();
    let abbreviated = match current.strip_prefix(&home) {
        Some(suffix) => format!("~{}", suffix),
        None => current,
    };
    Some(abbreviated)
}

pub fn get_prompt_symbol() -> char {
    unsafe {
        if libc::geteuid() == 0 {
            return '#';
        }
    }
    '$'
}
