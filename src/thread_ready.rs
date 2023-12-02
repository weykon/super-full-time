use ssh2_config::Host;
use std::net::TcpStream;
use std::path::Path;
use std::thread;

fn handle_ssh_connection(server: Host) -> Result<ssh2::Session, ssh2::Error> {
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
    Ok(session)
}

mod handle_channel;
use std::sync::mpsc;

pub struct ThreadBoot {
    name_mark: String,
    session: ssh2::Session,
    command_rx: mpsc::Receiver<String>,
    command_tx: mpsc::Sender<String>
}

pub fn threads_boot(entry_points: Vec<Host>) {
    let mut handles = vec![];
    for server in entry_points.iter() {
        let (tx, rx) = mpsc::channel();
        let server = server.clone();
        let mark_server_name = server.pattern[0].pattern.to_owned();
        let handle = thread::Builder::new()
            // 标记线程名称
            .name(mark_server_name.to_owned())
            .spawn(move || {
                let session = match handle_ssh_connection(server) {
                    Ok(sess) => sess,
                    Err(e) => {
                        println!("handle_ssh_connection error: {:?}", e);
                        return;
                    }
                };

                // 通道内
                handle_channel::operations(ThreadBoot {
                    name_mark: mark_server_name,
                    session,
                    command_rx: rx,
                    command_tx: tx,
                });
            })
            .unwrap();
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
