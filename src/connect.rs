use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use std::thread;
use ssh2_config::Host;

pub fn sshs(entry_points: Vec<Host>) {
    let mut handles = vec![];
    for server in entry_points.iter() {
        let server = server.clone();
        let handle = thread::spawn(move || {
            let mut session = ssh2::Session::new().unwrap();
            let key_file_path_str = server.params.identity_file.unwrap();
            println!("{:?}", key_file_path_str);
            let ip = server.params.host_name.as_ref().unwrap();
            let tcp = TcpStream::connect(format!("{}:22", ip)).unwrap();
            println!("connect to {} {}", ip, server.pattern[0].pattern);
            session.set_tcp_stream(tcp);
            session
                .userauth_pubkey_file("root", None, Path::new(&key_file_path_str[0]), None)
                .unwrap();
            assert!(session.authenticated());
            let mut channel = session.channel_session().unwrap();
            // 执行一个命令
            channel.exec("ls").unwrap();
            let mut output = String::new();
            channel.read_to_string(&mut output).unwrap();
            println!("{}", output);
            // 关闭通道
            channel.send_eof().unwrap();
            channel.wait_close().unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
