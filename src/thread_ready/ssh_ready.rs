use std::path::Path;

use std::net::TcpStream;

use ssh2_config::Host;

pub(crate) fn handle_ssh_connection(server: Host) -> Result<ssh2::Session, ssh2::Error> {
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
