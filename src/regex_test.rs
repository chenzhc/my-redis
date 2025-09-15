#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use log::info;
use regex::Regex;


pub fn test_regex() {
    let my_patter = "[A-Z]{1}[a-z]{2,8}";
    let input_text = "Nacy is going to the store";
    let name_regex = Regex::new(my_patter);
    if name_regex.is_err() {
        panic!("Error in regex pattern!");
    }

    let match_result = name_regex.unwrap().find(input_text);
    if let Some(my_match) = match_result {
        info!("Did input match pattern? {}", my_match.as_str());
    }
    
}


#[cfg(test)]
mod tests {
    use log::info;
    use super::*;

    #[tokio::test(flavor="multi_thread")]
    async fn it_piple_test() {
        crate::init();
        test_regex();

    }
}