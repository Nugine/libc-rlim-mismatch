#[test]
fn rlim_value() {
    use std::fs;
    use std::process::{Command, Stdio};

    fn exec(cmd: &[&str]) -> String {
        let mut child = Command::new(cmd[0]);
        child.args(&cmd[1..]);
        child.stdout(Stdio::piped());
        child.stderr(Stdio::inherit());
        let output = child.spawn().unwrap().wait_with_output().unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    const CPP_CODE: &str = r#"
        #include <iostream>
        #include <cstdint>
        #include <sys/resource.h>
        #include <typeinfo>
        using namespace std;
    
        int main(){
            cout<<RLIM_INFINITY<<'\n';
            cout<<RLIM_SAVED_CUR<<'\n';
            cout<<RLIM_SAVED_MAX<<'\n';
            cerr<<typeid(unsigned int).name()<<'\n';
            cerr<<typeid(unsigned long).name()<<'\n';
            cerr<<typeid(unsigned long long).name()<<'\n';
            cerr<<typeid(RLIM_INFINITY).name()<<'\n';
            cerr<<sizeof(RLIM_INFINITY)<<'\n';
            cerr<<sizeof(rlim_t)<<'\n';
            return 0;
        }
    "#;

    let cpp_path = "/tmp/__rlim_value_test.cpp";
    let exe_path = "/tmp/__rlim_value_test";
    fs::write(cpp_path, CPP_CODE).unwrap();

    exec(&["g++", cpp_path, "-std=c++11", "-o", exe_path]);

    let c_output = exec(&[exe_path]);

    let libc_output = format!(
        "{}\n{}\n{}\n",
        libc::RLIM_INFINITY,
        libc::RLIM_SAVED_CUR,
        libc::RLIM_SAVED_MAX,
    );

    assert_eq!(c_output, libc_output, "libc mismatch");

    fs::remove_file(cpp_path).ok();
    fs::remove_file(exe_path).ok();
}
