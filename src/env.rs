use std::env;

pub fn get_home_dir() -> Option<String> {
    env::var("HOME").ok()
}

pub fn get_current_dir() -> Option<String> {
    let current = env::current_dir().ok()?.to_string_lossy().into_owned();
    let home_path = get_home_dir()?;
    Some(
        current
            .strip_prefix(&home_path)
            .map_or_else(|| current.clone(), |s| format!("~{}", s)),
    )
}

pub fn get_prompt_symbol() -> char {
    unsafe {
        if libc::geteuid() == 0 {
            return '#';
        }
    }
    '$'
}
