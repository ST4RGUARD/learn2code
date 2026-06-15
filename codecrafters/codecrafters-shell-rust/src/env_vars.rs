// Gets environment variables, might use this to load .ashrc as well

pub fn get_vars(var: &str) -> String {
    match std::env::var(var) {
        Ok(val) => val,
        Err(e) => format!("couldn't interpret {}: {}",var,e),
    }
}
