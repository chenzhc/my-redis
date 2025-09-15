#![cfg_attr( debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code,unused_variables)]



#[cfg(test)]
mod tests {
    use std::{io::{Read, Write}, process::{self, Command}};

    use futures::AsyncWriteExt;
    use log::info;
    use crate::hello_server::process;

    use super::*;

    #[tokio::test(flavor="multi_thread")]
    async fn it_piple_test() {
        crate::init();
        let mut head_cmd = Command::new("head");
        head_cmd.arg("-n 1");
        head_cmd.stdin(std::process::Stdio::piped());
        head_cmd.stdout(std::process::Stdio::piped());

        let input_data = "inputone\ninputtwo".as_bytes();

        let mut proc_handle = head_cmd.spawn().unwrap();
        let mut stdin_hanle = proc_handle.stdin.take().unwrap();
        
        _ = stdin_hanle.write_all(input_data);
        
        _ = proc_handle.wait();
        let mut output_buf = String::new();
        let stdout_result = proc_handle.stdout.unwrap().read_to_string(&mut output_buf);
        info!("Result was : {}", output_buf);

    }

    #[test]
    fn it_spawn_test() {
        crate::init();

        let mut p2 = process::Command::new("echo");
        p2.env("FIRST_NAME", "test");
        p2.arg("$env:FIRST_NAME");
        
        // p2.stdout(std::process::Stdio::null());

        let mut p2_handle = p2.spawn().unwrap();

        info!("Doing some more work ...");
        let proc_result = p2_handle.wait().unwrap();
        info!("Exited with code: {:?}", proc_result.code().unwrap());

    }

    #[test]
    fn it_test01() {
        crate::init();
        info!("cmd test");
        let mut p1 = process::Command::new("which");
        p1.arg("python3");

        let proc_result = p1.output();
        if proc_result.is_ok() {
            let result = proc_result.ok().unwrap();

            info!("Was execution successfull? : {}", result.status.success());
            if !result.status.success() {
                info!("Error occurred: {}", result.status.code().unwrap());
            } else {
                let rs_str = String::from_utf8(result.stdout);
                info!("{}", rs_str.ok().unwrap());
            }
        }

        
    }
}