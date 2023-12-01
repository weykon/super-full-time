use ssh2_config::Host;
use std::alloc::handle_alloc_error;
use std::io::Read;
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

fn handle_chennel_operations(thread_boot_unit: Thread_Boot) {
    let session = thread_boot_unit.session;
    let mark_server_name = thread_boot_unit.name_mark;

    let mut channel = session.channel_session().unwrap();
    channel.exec("uname -a");
    channel.exec("last | head -n 5");
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    println!("{}", output);

    // 关闭通道
    channel.send_eof().unwrap();
    channel.wait_close().unwrap();
    println!("Exited: {}", mark_server_name);
}

struct Thread_Boot {
    name_mark : String, 
    session : ssh2::Session,
}

pub fn threads_boot(entry_points: Vec<Host>) {
    let mut handles = vec![];
    for server in entry_points.iter() {
        let server = server.clone();
        let mark_server_name = server.pattern[0].pattern.to_owned();
        let handle = thread::Builder::new()
            .name(mark_server_name.to_owned())
            .spawn(move || {
                let mut session = match handle_ssh_connection(server) {
                    Ok(sess) => sess,
                    Err(e) => {
                        println!("handle_ssh_connection error: {:?}", e);
                        return;
                    }
                };

                handle_chennel_operations(Thread_Boot { name_mark: mark_server_name, session });
            })
            .unwrap();
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
