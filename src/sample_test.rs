#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]



fn get_full_name(first: &str, last: &str) -> String {
    let pattern = &['*', '$', '^', '|', '+', '-'];
    if first.contains(pattern) {
        panic!("First name cannot contain special characters!");
    }
    if last.contains(pattern) {
        panic!("Last name cannot contain special characters!");
    }
    let mut result = "".to_string();

    result.push_str(first);
    result.push_str(" ");
    result.push_str(last);

    return result;
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    #[should_panic]
    fn it_get_full_name_special_chars() {
        crate::init();
        _ = get_full_name("*Tr&ev^or", "Sullivan");

    }

    #[test]
    #[should_panic]
    fn it_get_full_name_special_chars_last() {
        crate::init();
        _ = get_full_name("Trevor", "Sul*l&ivan");

    }

    #[test]
    fn it_test01() {
        crate::init();
        info!("test");

        let rs_str = get_full_name("test01", "last");
        info!("{}", rs_str);

    }
}