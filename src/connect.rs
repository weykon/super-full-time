use ssh2_config::Host;
use std::net::TcpStream;
use std::path::Path;
use std::thread;

pub fn sshs(entry_points: Vec<Host>) {
    let mut handles = vec![];
    for server in entry_points.iter() {
        let server = server.clone();
        let mark_server_name = server.pattern[0].pattern.to_owned();
        let handle = thread::Builder::new()
            .name(mark_server_name.to_owned())
            .spawn(move || {
                let mut session = ssh2::Session::new().unwrap();
                let key_file_path_str = server.params.identity_file.unwrap();
                println!("{:?}", key_file_path_str);
                let ip = server.params.host_name.as_ref().unwrap();
                let tcp = TcpStream::connect(format!("{}:22", ip)).unwrap();
                println!("connect to {} {}", ip, server.pattern[0].pattern);

                session.set_tcp_stream(tcp);
                session.handshake().unwrap();

                let private_key_path = Path::new(&key_file_path_str[0]);
                println!("private_key_path : {:?}", private_key_path);

                match session.userauth_pubkey_file("root", None, private_key_path, None) {
                    Ok(_) => println!("userauth_pubkey_file success"),
                    Err(e) => println!("userauth_pubkey_file error: {:?}", e),
                }

                assert!(session.authenticated());

                let mut channel = session.channel_session().unwrap();
                use super::process::common;
                common::common(&mut channel);
                
                // 关闭通道
                channel.send_eof().unwrap();
                channel.wait_close().unwrap();
                println!("Exited: {}", mark_server_name);
            })
            .unwrap();
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
